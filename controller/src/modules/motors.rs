use std::sync::Arc;

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
use parking_lot::Mutex;
use tokio::io::AsyncWriteExt;
use tokio_serial::{self, SerialPortBuilderExt};

pub struct Motors {
	serial: tokio_serial::SerialStream,
}

impl Motors {
	pub fn new(
		config::Motors {
			baud_rate,
			uart_path,
			..
		}: config::Motors,
	) -> Result<Motors> {
		let serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;
		Ok(Motors { serial })
	}
}

#[async_trait]
impl Module for Motors {
	async fn tick(&mut self, state: &mut Arc<Mutex<State>>) -> Result<()> {
		let motor_commands = {
			let mut state = state.lock();
			let config::Motors {
				motor_offset,
				speed,
				..
			} = state.config.motors.as_ref().unwrap().to_owned();

			if let Some(vec) = state.line_vector {
				state.move_vector = {
					let before_projection = state.ball_follow_vector;

					Some(vec * (dot(before_projection, vec) / dot(vec, vec)))
				};
			}

			state.ball_follow_vector = Vec2 { x: 1.0, y: 0.0 };
			match state.move_vector {
				Some(vec) => {
					let move_angle = vec.angle_rad();
					let left_offset = move_angle - motor_offset;
					let right_offset = move_angle + motor_offset;

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
					.map(|x| x * speed)
					.map(|x| x.map_range((-1.0, 1.0), (0.0, 253.0)) as u8)
				}
				None => [0, 0, 0, 0].map(|x| x.map_range((-1, 1), (0, 253)) as u8),
			}
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
