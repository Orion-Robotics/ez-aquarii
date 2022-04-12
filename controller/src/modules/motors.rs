use anyhow::Result;
use async_trait::async_trait;
use tokio_serial::{self, SerialPortBuilderExt};

use super::{state::State, Module};

pub struct Motors {
	serial: tokio_serial::SerialStream,
}

impl Motors {
	pub async fn new(path: String, baud_rate: u32) -> Result<Motors> {
		let serial = tokio_serial::new(path, baud_rate).open_native_async()?;
		Ok(Motors { serial })
	}
}

#[async_trait]
impl Module for Motors {
	async fn tick(&mut self, state: &mut State) -> Result<()> {}

	fn name(&self) -> &'static str {
		"Motors"
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
