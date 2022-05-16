use std::sync::Arc;

use crate::{
	config::{self},
	modules,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::Mutex;
use tokio::fs::{File, OpenOptions};

use crate::ipc;

use super::{
	state::{ModuleSync, State},
	Module,
};

pub struct Camera {
	pub socket_file: File,
}

impl Camera {
	pub async fn new(config::Camera { path, .. }: config::Camera) -> Result<Self> {
		Ok(Camera {
			socket_file: OpenOptions::new()
				.read(true)
				.write(true)
				.create(true)
				.mode(0o600)
				.open(path)
				.await
				.with_context(|| "failed to open file")?,
		})
	}
}

#[async_trait]
impl Module for Camera {
	fn name(&self) -> &'static str {
		"camera"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>, sync: &mut ModuleSync) -> Result<()> {
		let data = ipc::read_msgpack::<modules::state::CameraMessage, _>(&mut self.socket_file)
			.await
			.with_context(|| "failed to read packet")?;
		tracing::debug!("received camera packet: {:?}", data);
		state.lock().data.camera_data = data;
		sync.camera_notify.notify_waiters();
		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
