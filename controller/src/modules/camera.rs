use std::sync::Arc;

use crate::{
	config::{self},
	modules,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use num::traits::Pow;
use parking_lot::Mutex;
use tokio::fs::{File, OpenOptions};

use crate::ipc;

use super::{state::State, Module};

pub struct Camera {
	pub socket_file: Option<File>,
}

impl Camera {
	pub async fn new(
		config::Camera {
			enable_reading,
			path,
			..
		}: config::Camera,
	) -> Result<Self> {
		let f = if enable_reading {
			Some(
				OpenOptions::new()
					.read(true)
					.write(true)
					.create(true)
					.mode(0o600)
					.open(path)
					.await
					.with_context(|| "failed to open file")?,
			)
		} else {
			None
		};
		Ok(Camera { socket_file: f })
	}
}

#[async_trait]
impl Module for Camera {
	fn name(&self) -> &'static str {
		"camera"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>) -> Result<()> {
		if let Some(ref mut file) = self.socket_file {
			let data = ipc::read_msgpack::<modules::state::CameraMessage, _>(file)
				.await
				.with_context(|| "failed to read packet")?;
			tracing::debug!("received camera packet: {:?}", data);
			state.lock().data.camera_data = data;
		}
		let (camera_config, data) = {
			let state = state.lock();
			(
				state.config.camera.as_ref().unwrap().to_owned(),
				state.data.camera_data,
			)
		};
		let orbit_offset = {
			let config::OrbitConfig {
				curve_steepness,
				shift_x,
				shift_y,
			} = camera_config.orbit;
			orbit(data.angle, curve_steepness, shift_x, shift_y)
		};

		let dampen_amount = orbit_offset * {
			let config::DampenConfig {
				curve_steepness,
				shift_x,
				shift_y,
			} = camera_config.dampen;
			dampen(data.angle, curve_steepness, shift_x, shift_y)
		};

		let orbit_angle = data.angle + (orbit_offset * dampen_amount);

		{
			let mut state = state.lock();
			state.orbit_offset = orbit_offset;
			state.dampen_amount = dampen_amount;
			state.orbit_angle = orbit_angle;
		}

		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}

/// Orbit contains the function that offsets the robot's angle by an amount depending on distance to an object.
/// https://www.desmos.com/calculator/c6d8zvyw5z
/// - angle: the angle of the ball (in trig plane)
/// - curve_steepness: the exponent that makes the slope of the orbit steeper.
/// - shift_x: how far to shift the orbit function left of right.
/// - shift_y: how far to shift the orbit function up and down.
/// Returns the angle
fn orbit(angle: f64, curve_steepness: f64, shift_x: f64, shift_y: f64) -> f64 {
	90.0f64.min(curve_steepness.pow(angle + shift_x) - shift_y)
}

/// Returns the amount to scale the orbit function by, from 0 to 1.
/// https://www.desmos.com/calculator/4ci4hifjf3
/// This is used to make the orbit function apply less when the ball is far away,
/// so it can go in a straight line instead of a curve.
/// - distance: the distance from the robot to the ball (in an arbritrary unit determined by the camera)
/// - curve_steepness: the exponent that makes the slope of the dampen steeper.
/// - shift_x: how far to shift the orbit function left of right.
/// - shift_y: how far to shift the orbit function up and down.
fn dampen(distance: f64, curve_steepness: f64, shift_x: f64, shift_y: f64) -> f64 {
	1.0f64.min(curve_steepness.pow(shift_x + distance) - shift_y)
}
