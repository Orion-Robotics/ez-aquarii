use std::net::SocketAddr;

use super::{state::State, Module};
use anyhow::Result;
use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ws::WebSocketUpgrade, Extension};
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

pub struct StateRecorder {
	sender: broadcast::Sender<State>,
	kill_sender: Option<oneshot::Sender<bool>>,
}

impl StateRecorder {
	pub async fn new(addr: String) -> Result<Self> {
		let (sender, _) = broadcast::channel(1);
		let addr = addr.parse::<SocketAddr>()?;

		let (kill_sender, kill_receiver) = oneshot::channel();

		let app = Router::new()
			.route("/state", get(websocket_handler))
			.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
			.layer(Extension(sender.clone()));

		tokio::spawn(
			axum::Server::bind(&addr)
				.serve(app.into_make_service())
				.with_graceful_shutdown(async {
					kill_receiver.await.ok();
				}),
		);

		Ok(Self {
			sender,
			kill_sender: Some(kill_sender),
		})
	}
}

#[async_trait]
impl Module for StateRecorder {
	fn name(&self) -> &'static str {
		"StateRecorder"
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		if let Some(kill_sender) = self.kill_sender.take() {
			kill_sender
				.send(true)
				.map_err(|_| anyhow::anyhow!("Error sending kill signal"))?;
		}
		Ok(())
	}

	async fn tick(&mut self, robot_state: &mut State) -> Result<()> {
		if let Err(err) = self.sender.send(robot_state.clone()) {
			tracing::trace!("Error broadcasting new state: {:?}", err);
		}
		Ok(())
	}
}
