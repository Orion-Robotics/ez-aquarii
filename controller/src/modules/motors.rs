use std::f64::consts::PI;

use crate::{
	config::{self},
	math::{
		map_range::MapRange,
		vec2::{dot, Vec2},
	},
};

use super::{state::State, Module};
use anyhow::Result;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio_serial::{self, SerialPortBuilderExt};

pub struct Motors {
	serial: tokio_serial::SerialStream,
	motor_offset: f64,
	speed: f64,
	pub angle: f64,
}

impl Motors {
	pub async fn new(
		config::Motors {
			baud_rate,
			motor_offset,
			uart_path,
			speed,
		}: config::Motors,
	) -> Result<Motors> {
		let serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;
		Ok(Motors {
			serial,
			motor_offset,
			speed,
			angle: 0.0,
		})
	}
}

#[async_trait]
impl Module for Motors {
	async fn tick(&mut self, state: &mut State) -> Result<()> {
		// state.move_vector = {
		// 	let before_projection = state.ball_follow_vector;

		// 	state.line_vector
		// 		* (dot(before_projection, state.line_vector)
		// 			/ dot(state.line_vector, state.line_vector))
		// };

		// state.move_vector = state.line_vector * -1.0;

		state.move_vector = Vec2 { x: 0.0, y: -1.0 };

		let move_angle = state.move_vector.angle_rad();
		let left_offset = move_angle - self.motor_offset;
		let right_offset = move_angle + self.motor_offset;

		let motor_commands = {
			let front_right = right_offset.sin();
			let back_right = -right_offset.sin();
			let back_left = -left_offset.sin();
			let front_left = left_offset.sin();

			// motor power optimization
			let max_power = front_right
				.abs()
				.max(back_left.abs())
				.max(front_left.abs())
				.max(back_right.abs());

			[
				-front_left / max_power,
				front_right / max_power,
				back_right / max_power,
				-back_left / max_power,
			]
			.map(|x| x * self.speed)
			.map(|x| x.map_range((-1.0, 1.0), (0.0, 253.0)) as u8)
		};

		// let motor_commands = [127, 127, 127, 253];

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
