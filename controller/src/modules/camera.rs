use crate::{config, math::vec2::Vec2, modules};
use anyhow::{Context, Result};
use async_trait::async_trait;
use num::traits::Pow;
use tokio::fs::{File, OpenOptions};

use crate::ipc;

use super::{state::State, Module};

pub struct Camera {
	pub socket_file: File,
	pub orbit: config::OrbitConfig,
	pub dampen: config::DampenConfig,
}

impl Camera {
	pub async fn new(
		config::Camera {
			path,
			orbit,
			dampen,
		}: config::Camera,
	) -> Result<Camera> {
		let f = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.mode(0o600)
			.open(path)
			.await
			.with_context(|| "failed to open file")?;
		Ok(Camera {
			socket_file: f,
			dampen,
			orbit,
		})
	}
}

#[async_trait]
impl Module for Camera {
	async fn tick(&mut self, state: &mut State) -> Result<()> {
		let data = ipc::read_msgpack::<modules::state::CameraMessage, _>(&mut self.socket_file)
			.await
			.with_context(|| "failed to read packet")?;

		let orbit_offset = {
			let config::OrbitConfig {
				curve_steepness,
				shift_x,
				shift_y,
			} = self.orbit;
			orbit(data.angle, curve_steepness, shift_x, shift_y)
		};

		let dampen_amount = orbit_offset * {
			let config::DampenConfig {
				curve_steepness,
				shift_x,
				shift_y,
			} = self.dampen;
			dampen(data.angle, curve_steepness, shift_x, shift_y)
		};

		let orbit_angle = data.angle + (orbit_offset * dampen_amount);

		state.data.camera_data = data;
		state.orbit_offset = orbit_offset;
		state.dampen_amount = dampen_amount;
		state.orbit_angle = orbit_angle;

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
