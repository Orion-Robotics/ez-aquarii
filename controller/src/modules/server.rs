use std::{net::SocketAddr, sync::Arc, time::Duration};

use super::{
	state::{ModuleSync, State},
	Module,
};
use crate::config::{self, Config};
use anyhow::Result;
use async_trait::async_trait;
use axum::{
	body::StreamBody,
	extract::{
		ws::{Message, WebSocket, WebSocketUpgrade},
		Extension,
	},
	http::{Response, StatusCode},
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use futures::{stream::StreamExt, SinkExt};
use opencv::{core::Size, imgcodecs, imgproc, prelude::Mat};
use parking_lot::RwLock;
use tokio::{
	sync::{broadcast, mpsc, oneshot},
	time::{interval, Interval},
};
use tower_http::cors::{Any, CorsLayer};

async fn websocket_handler(
	ws: WebSocketUpgrade,
	Extension(state): Extension<(broadcast::Sender<State>, mpsc::Sender<Config>)>,
) -> impl IntoResponse {
	ws.on_upgrade(|socket| async move {
		if let Err(e) = websocket(socket, state.0, state.1).await {
			tracing::error!("Error handling websocket: {:?}", e)
		}
	})
}

async fn websocket(
	stream: WebSocket,
	state: broadcast::Sender<State>,
	sender: mpsc::Sender<Config>,
) -> Result<()> {
	let mut subscriber = state.subscribe();
	let (mut tx, mut rx) = stream.split();

	tokio::spawn(async move {
		loop {
			if let Ok(state) = subscriber.recv().await {
				if let Ok(message) = serde_json::to_string(&state) {
					let _err = tx.send(Message::Text(message)).await;
				}
			}
		}
	});

	loop {
		if let Some(Ok(message)) = rx.next().await {
			let res = serde_json::from_slice(&message.into_data())?;
			sender.send(res).await?;
		}
	}
}

async fn get_config(Extension(config): Extension<Config>) -> impl IntoResponse {
	Json(config)
}

async fn stream_mjpeg(
	Extension(sender): Extension<broadcast::Sender<Vec<u8>>>,
) -> impl IntoResponse {
	let receiver = sender.subscribe();
	let stream = tokio_stream::wrappers::BroadcastStream::new(receiver)
		.map(|result| {
			result.map(|bytes| {
				tracing::debug!("successfully received image over channel");
				[
					b"--FRAME\r\nContent-Type: image/jpeg\r\nContent-Length: ",
					bytes.len().to_string().as_bytes(),
					b"\r\n",
					b"\r\n",
					bytes.as_slice(),
					b"\r\n",
				]
				.concat()
			})
		})
		.filter_map(|res| async move {
			if res.is_err() {
				None
			} else {
				Some(res)
			}
		});
	// this can only fail if the builder response was misconfigured
	Response::builder()
		.status(StatusCode::OK)
		.header(
			"Content-Type",
			"multipart/x-mixed-replace; boundary=--FRAME",
		)
		.header("Pragma", "no-cache")
		.header("Cache-Control", "no-cache, private")
		.header("Age", "0")
		.body(StreamBody::new(stream))
		.unwrap()
}

pub struct StateRecorder {
	state_sender: broadcast::Sender<State>,
	client_message_receiver: mpsc::Receiver<Config>,
	client_message_sender: mpsc::Sender<Config>,
	kill_sender: Option<oneshot::Sender<()>>,
	kill_receiver: Option<oneshot::Receiver<()>>,
	image_chan: (broadcast::Sender<Vec<u8>>, broadcast::Receiver<Vec<u8>>),
	addr: SocketAddr,
	config: Config,
	state_interval: Interval,
}

impl StateRecorder {
	pub async fn new(config: Config, config::Server { addr }: config::Server) -> Result<Self> {
		let (sender, _) = broadcast::channel(1);
		let (client_message_sender, client_message_receiver) = mpsc::channel(1);
		let addr = addr.parse::<SocketAddr>()?;

		let (kill_sender, kill_receiver) = oneshot::channel();

		Ok(Self {
			state_sender: sender,
			client_message_receiver,
			client_message_sender,
			kill_sender: Some(kill_sender),
			kill_receiver: Some(kill_receiver),
			image_chan: broadcast::channel(1),
			addr,
			config,
			state_interval: interval(Duration::from_millis(10)),
		})
	}
}

#[async_trait]
impl Module for StateRecorder {
	fn name(&self) -> &'static str {
		"server"
	}

	async fn start(&mut self) -> Result<()> {
		let app = Router::new()
			.route("/state", get(websocket_handler))
			.route("/config", get(get_config))
			.route("/camera", get(stream_mjpeg))
			.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
			.layer(Extension((
				self.state_sender.clone(),
				self.client_message_sender.clone(),
			)))
			.layer(Extension(self.config.clone()))
			.layer(Extension(self.image_chan.0.clone()));

		let kill_receiver = self.kill_receiver.take().unwrap();
		let addr = self.addr;

		tokio::spawn(async move {
			axum::Server::bind(&addr)
				.serve(app.into_make_service())
				.with_graceful_shutdown(async {
					kill_receiver.await.ok();
				})
				.await
				.unwrap();
		});

		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		self.kill_sender
			.take()
			.unwrap()
			.send(())
			.map_err(|_| anyhow::anyhow!("Error sending kill signal"))?;
		Ok(())
	}

	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		tokio::select! {
				  Some(msg) = self.client_message_receiver.recv() => {
			state.write().config = msg.clone();
			msg.save("./config.yaml").await?;
		  },
				_ = sync.camera_notify.notified() => {
			let mut buf = opencv::core::Vector::new();
			let mut scaled = Mat::default();
			imgproc::resize(&sync.frame.lock().clone(), &mut scaled, Size::default(), 1.0, 1.0, 0)?;
			imgcodecs::imencode(".jpg", &scaled, &mut buf, &opencv::core::Vector::new())?;
			self.image_chan.0.send(buf.into())?;
				}
		  _ = self.state_interval.tick() => {
		  let _err = self.state_sender.send(state.read().clone());
		}
		  }
		Ok(())
	}
}
