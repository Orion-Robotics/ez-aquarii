use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use parking_lot::Mutex;
use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::config;

use super::{
	state::{ModuleSync, State},
	Module,
};

pub struct Reader {
	pub serial: SerialStream,
}

impl Reader {
	pub fn new(
		config::Reader {
			baud_rate,
			uart_path,
			..
		}: config::Reader,
	) -> Result<Self> {
		let serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;

		Ok(Reader { serial })
	}
}

#[async_trait]
impl Module for Reader {
	fn name(&self) -> &'static str {
		"reader"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>, sync: &mut ModuleSync) -> Result<()> {
		let config::Reader {
			line_sensor_count, ..
		} = state.lock().config.reader.as_ref().unwrap().to_owned();
		while self.serial.read_u8().await? != 255 {}
		let angle = self.serial.read_f32_le().await?;
		let mut raw_data: Vec<u8> = vec![0; line_sensor_count];
		self.serial.read_exact(&mut raw_data).await?;
		raw_data.reverse();
		let mut state = state.lock();
		state.data.sensor_data = raw_data;
		state.data.orientation = angle;
		sync.reader_notify.notify_waiters();
		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
