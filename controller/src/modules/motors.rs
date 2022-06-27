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
	) -> Result<Motors> {
		let serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;
		Ok(Motors { serial })
	}
}

#[async_trait]
impl Module for Motors {
	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		let motor_commands = {
			let state = state.write();
			let config::Motors {
				motor_offset,
				speed,
				rotation_scalar,
				..
			} = state.config.motors.as_ref().unwrap().to_owned();

			// the rotation is in range -180 to 180
			// this will scale it to -1 to 1
			let scaled_rotation = state.rotation / PI * rotation_scalar;

			if speed > 1.0 {
				tracing::error!("speeds are from 0 to 1!");
			}

			let powers = match state.move_vector {
				Some(vec) => {
					let move_angle = vec.angle_rad();
					let left_offset = move_angle - motor_offset;
					let right_offset = move_angle + motor_offset;

					[
						-right_offset.sin(),
						right_offset.sin(),
						left_offset.sin(),
						-left_offset.sin(),
					]
				}
				None => [0.0, 0.0, 0.0, 0.0],
			}
			.map(|power| power + scaled_rotation);
			// .map(|power| power.max(-1.0).min(1.0));

			// motor power optimization
			let max_power = *powers
				.map(|power| power.abs())
				.iter()
				.min_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap();

			// let percentages = powers.map(|power| power / max_power);

			powers
				.map(|x| x * speed)
				.map(|x| x.map_range((-1.0, 1.0), (0.0, 253.0)) as u8)
		};
		// self.serial.write_all(&[127, 127, 253, 127]).await?;
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
