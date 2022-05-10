use std::net::SocketAddr;

use super::{state::State, Module};
use crate::config::{self, Config};
use anyhow::Result;
use async_trait::async_trait;
use axum::{
	extract::{
		ws::{Message, WebSocket, WebSocketUpgrade},
		Extension,
	},
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc, oneshot};
use tower_http::cors::{Any, CorsLayer};

async fn websocket_handler(
	ws: WebSocketUpgrade,
	Extension(state): Extension<(broadcast::Sender<State>, mpsc::Sender<Config>)>,
) -> impl IntoResponse {
	tracing::warn!("woozy");
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
					let _ = tx.send(Message::Text(message)).await;
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
pub struct StateRecorder {
	state_sender: broadcast::Sender<State>,
	client_message_receiver: mpsc::Receiver<Config>,
	client_message_sender: mpsc::Sender<Config>,
	kill_sender: Option<oneshot::Sender<()>>,
	kill_receiver: Option<oneshot::Receiver<()>>,
	kill_complete_sender: Option<oneshot::Sender<()>>,
	kill_complete_receiver: Option<oneshot::Receiver<()>>,
	addr: SocketAddr,
	config: Config,
}

impl StateRecorder {
	pub async fn new(config: Config, config::Server { addr }: config::Server) -> Result<Self> {
		let (sender, _) = broadcast::channel(1);
		let (client_message_sender, client_message_receiver) = mpsc::channel(1);
		let addr = addr.parse::<SocketAddr>()?;

		let (kill_sender, kill_receiver) = oneshot::channel();
		let (kill_complete_sender, kill_complete_receiver) = oneshot::channel();

		Ok(Self {
			state_sender: sender,
			client_message_receiver,
			client_message_sender,
			kill_sender: Some(kill_sender),
			kill_receiver: Some(kill_receiver),
			kill_complete_sender: Some(kill_complete_sender),
			kill_complete_receiver: Some(kill_complete_receiver),
			addr,
			config,
		})
	}
}

#[async_trait]
impl Module for StateRecorder {
	fn name(&self) -> &'static str {
		"StateRecorder"
	}

	async fn start(&mut self) -> Result<()> {
		let app = Router::new()
			.route("/state", get(websocket_handler))
			.route("/config", get(get_config))
			.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
			.layer(Extension((
				self.state_sender.clone(),
				self.client_message_sender.clone(),
			)))
			.layer(Extension(self.config.clone()));

		let kill_receiver = self.kill_receiver.take().unwrap();
		let kill_complete_sender = self.kill_complete_sender.take().unwrap();
		let addr = self.addr;

		tokio::spawn(async move {
			axum::Server::bind(&addr)
				.serve(app.into_make_service())
				.with_graceful_shutdown(async {
					kill_receiver.await.ok();
				})
				.await
				.unwrap();
			tracing::debug!("server has shut down");
			kill_complete_sender.send(()).ok();
		});
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		self.kill_sender
			.take()
			.unwrap()
			.send(())
			.map_err(|_| anyhow::anyhow!("Error sending kill signal"))?;
		self.kill_complete_receiver
			.take()
			.unwrap()
			.await
			.map_err(|_| anyhow::anyhow!("Error waiting for kill_complete_sender"))?;
		Ok(())
	}

	async fn tick(&mut self, state: &mut State) -> Result<()> {
		if let Err(err) = self.state_sender.send(state.clone()) {
			tracing::trace!("Error broadcasting new state: {:?}", err);
		}
		if let Ok(msg) = self.client_message_receiver.try_recv() {
			println!("{:?}", msg);
			state.config = msg;
		}
		Ok(())
	}
}
