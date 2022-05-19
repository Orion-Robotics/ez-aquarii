use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use bytes::BytesMut;
use futures::StreamExt;
use parking_lot::Mutex;
use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use crate::config;

use super::{
	state::{ModuleSync, State},
	Module,
};

pub struct Reader {
	codec: Framed<SerialStream, ReaderCodec>,
}

impl Reader {
	pub async fn new(
		config::Reader {
			baud_rate,
			uart_path,
			..
		}: config::Reader,
	) -> Result<Self> {
		let mut serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;
		while tokio::io::AsyncReadExt::read_u8(&mut serial)
			.await
			.context("failed to find start of next message")?
			!= b'\n'
		{}
		let codec = ReaderCodec.framed(serial);
		Ok(Reader { codec })
	}
}

#[async_trait]
impl Module for Reader {
	fn name(&self) -> &'static str {
		"reader"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>, sync: &mut ModuleSync) -> Result<()> {
		if let Some(res) = self.codec.next().await {
			let (angle, sensors) = res?;
			let mut state = state.lock();
			state.data.sensor_data = sensors;
			state.data.orientation = angle as f64;
			if state.initial_orientation.is_none() {
				tracing::info!("initial orientation set to {}", angle);
				state.initial_orientation = Some(angle.into());
			}
		}

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

struct ReaderCodec;

impl Decoder for ReaderCodec {
	type Item = (f32, Vec<u8>);
	type Error = anyhow::Error;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		let newline = src.as_ref().iter().position(|b| *b == b'\n');
		if let Some(n) = newline {
			let mut line = src.split_to(n + 1);
			line.truncate(line.len() - 3);
			return match std::str::from_utf8(line.as_ref()) {
				Ok(s) => {
					let mut parts = s.split(' ');
					let angle: f32 = parts
						.next()
						.ok_or(anyhow::anyhow!("failed to parse angle"))?
						.parse()?;
					let line_sensors = parts
						.map(|s| Ok(s.parse::<u8>()?))
						.collect::<Result<Vec<u8>, anyhow::Error>>()?;
					Ok(Some((angle, line_sensors)))
				}
				Err(_) => Err(anyhow::anyhow!("Invalid String")),
			};
		}
		Ok(None)
	}
}
