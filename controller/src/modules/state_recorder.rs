use std::net::SocketAddr;

use super::{state::State, Module};
use crate::config::Config;
use anyhow::Result;
use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ws::WebSocketUpgrade, Extension};
use axum::Json;
use axum::{response::IntoResponse, routing::get, Router};
use tokio::sync::{broadcast, oneshot};
use tower_http::cors::{Any, CorsLayer};

async fn websocket_handler(
	ws: WebSocketUpgrade,
	Extension(state): Extension<broadcast::Sender<State>>,
) -> impl IntoResponse {
	tracing::warn!("woozy");
	ws.on_upgrade(|socket| async move {
		if let Err(e) = websocket(socket, state).await {
			tracing::error!("Error handling websocket: {:?}", e)
		}
	})
}

async fn websocket(mut stream: WebSocket, state: broadcast::Sender<State>) -> Result<()> {
	let mut subscriber = state.subscribe();
	loop {
		if let Ok(state) = subscriber.recv().await {
			stream
				.send(Message::Text(serde_json::to_string(&state)?))
				.await?;
		}
	}
}

async fn get_config(Extension(config): Extension<Config>) -> impl IntoResponse {
	Json(config)
}
pub struct StateRecorder {
	sender: broadcast::Sender<State>,
	kill_sender: Option<oneshot::Sender<()>>,
	kill_receiver: Option<oneshot::Receiver<()>>,
	kill_complete_sender: Option<oneshot::Sender<()>>,
	kill_complete_receiver: Option<oneshot::Receiver<()>>,
	addr: SocketAddr,
	config: Config,
}

impl StateRecorder {
	pub async fn new(config: Config, addr: String) -> Result<Self> {
		let (sender, _) = broadcast::channel(1);
		let addr = addr.parse::<SocketAddr>()?;

		let (kill_sender, kill_receiver) = oneshot::channel();
		let (kill_complete_sender, kill_complete_receiver) = oneshot::channel();

		Ok(Self {
			sender,
			kill_sender: Some(kill_sender),
			kill_receiver: Some(kill_receiver),
			kill_complete_sender: Some(kill_complete_sender),
			kill_complete_receiver: Some(kill_complete_receiver),
			addr,
			config: config.clone(),
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
			.layer(Extension(self.sender.clone()))
			.layer(Extension(self.config.clone()));
		let addr = self.addr.clone();
		let kill_receiver = self.kill_receiver.take().unwrap();
		let kill_complete_sender = self.kill_complete_sender.take().unwrap();

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

	async fn tick(&mut self, robot_state: &mut State) -> Result<()> {
		if let Err(err) = self.sender.send(robot_state.clone()) {
			tracing::trace!("Error broadcasting new state: {:?}", err);
		}
		Ok(())
	}
}
