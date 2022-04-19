use super::super::math::map_range::MapRange;
use super::{state::State, Module};
use anyhow::Result;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio_serial::{self, SerialPortBuilderExt};

pub struct Motors {
	serial: tokio_serial::SerialStream,
	motor_offset: f64,
}

impl Motors {
	pub async fn new(path: String, baud_rate: u32, motor_offset: f64) -> Result<Motors> {
		let serial = tokio_serial::new(path, baud_rate).open_native_async()?;
		Ok(Motors {
			serial,
			motor_offset,
		})
	}
}

#[async_trait]
impl Module for Motors {
	async fn tick(&mut self, state: &mut State) -> Result<()> {
		let move_angle = state.move_vector.angle_rad();
		let left_offset = move_angle - self.motor_offset;
		let right_offset = move_angle + self.motor_offset;

		let motor_commands = {
			let front_right = -left_offset.sin();
			let back_left = left_offset.sin();
			let front_left = right_offset.sin();
			let back_right = -right_offset.sin();

			// motor power optimization
			let max_power = front_right
				.abs()
				.max(back_left.abs())
				.max(front_left.abs())
				.max(back_right.abs());

			[
				front_right / max_power,
				front_left / max_power,
				back_left / max_power,
				back_right / max_power,
			]
			.map(|x| x.map_range((-1.0, 1.0), (0.0, 253.0)) as u8)
		};

		tracing::debug!("motor_commands: {:?}", motor_commands);

		self.serial.write_all(&motor_commands).await?;

		self.serial.write_u8(255).await?;

		Ok(())
	}

	fn name(&self) -> &'static str {
		"motors"
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
