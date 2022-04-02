use crate::comms;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};

use crate::ipc;

use super::{state::State, Module};

pub struct Camera {
	pub socket_file: File,
}

impl Camera {
	pub async fn new(path: PathBuf) -> Result<Camera> {
		let f = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.mode(0o600)
			.open(path)
			.await
			.with_context(|| "failed to open file")?;
		Ok(Camera { socket_file: f })
	}
}

#[async_trait]
impl Module for Camera {
	async fn tick(&mut self, _state: &mut State) -> Result<()> {
		let data = ipc::read_proto::<comms::Packet, _>(&mut self.socket_file)
			.await
			.with_context(|| "failed to read packet")?;
		println!("{:?}", data.time);
		Ok(())
	}

	fn name(&self) -> &'static str {
		"camera"
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
