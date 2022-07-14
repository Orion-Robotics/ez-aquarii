use std::{f64::consts::PI, sync::Arc};

use crate::{
	config::{self},
	math::{
		map_range::MapRange,
		vec2::{dot, Vec2},
	},
};

use super::{
	state::{ModuleSync, State},
	Module,
};
use anyhow::Result;
use async_trait::async_trait;
use parking_lot::{Mutex, RwLock};
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
	) -> Result<Self> {
		let mut serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;
		serial.set_exclusive(false)?;
		Ok(Self { serial })
	}
}

#[async_trait]
impl Module for Motors {
	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		let motor_commands = {
			let config::Motors {
				motor_offset,
				speed,
				rotation_scalar,
				rotation_slope,
				..
			} = state.read().config.motors.as_ref().unwrap().clone();

			// the rotation is in range -180 to 180
			// this will scale it to -1 to 1
			let scaled_rotation = state.read().rotation / PI;
			// scale it to a rotation curve
			let scaled_rotation =
				scaled_rotation.signum() * rotation_slope * scaled_rotation.abs().sqrt();
			state.write().scaled_rotation = scaled_rotation;

			if speed > 1.0 {
				tracing::error!("speeds are from 0 to 1!");
			}

			// determines if you should be going full speed
			let go_full_speed = state
				.read()
				.move_vector
				.map(|(_, go_full_speed)| go_full_speed)
				.unwrap_or(false);

			let powers = match state.read().move_vector {
				Some((vec, _)) => {
					let move_angle = vec.angle_rad();
					let left_offset = move_angle - motor_offset;
					let right_offset = move_angle + motor_offset;

					[
						left_offset.sin(),
						-right_offset.sin(),
						-left_offset.sin(),
						right_offset.sin(),
					]
				}
				None => [0.0, 0.0, 0.0, 0.0],
			}
			.map(|power| power + scaled_rotation);
			// .map(|power| power.max(-1.0).min(1.0));

			// motor power optimization
			// let max_power = *powers
			// 	.map(f64::abs)
			// 	.iter()
			// 	.min_by(|a, b| a.partial_cmp(b).unwrap())
			// 	.unwrap();

			// let percentages = powers.map(|power| power / max_power);
			let powers = if go_full_speed {
				powers
			} else {
				powers.map(|x| x * speed)
			};
			powers.map(|x| x.map_range((-1.0, 1.0), (0.0, 253.0)) as u8)
		};
		let motor_commands = if state.read().paused {
			[127, 127, 127, 127]
		} else {
			motor_commands
		};
		// self.serial.write_all(&[191, 253, 0, 63]).await?;
		self.serial.write_all(&motor_commands).await?;
		self.serial.write_u8(255).await?;
		state.write().motor_commands = Vec::from(motor_commands);

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
