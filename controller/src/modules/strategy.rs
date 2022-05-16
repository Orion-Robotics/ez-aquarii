use std::{f64::consts::PI, sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use futures::{future::select_all, select, FutureExt};
use num::traits::Pow;
use parking_lot::Mutex;

use crate::{
	config,
	math::{
		angles::{make_bipolar, true_angle},
		vec2::Vec2,
	},
};

use super::{
	state::{self, CameraMessage, ModuleSync, State},
	Module,
};

pub struct Strategy {}

#[async_trait]
impl Module for Strategy {
	fn name(&self) -> &'static str {
		"strategy"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>, sync: &mut ModuleSync) -> Result<()> {
		// sync.reader_notify.notified().await;
		select_all(vec![
			sync.camera_notify.notified().boxed(),
			sync.reader_notify.notified().boxed(),
		])
		.await;
		let mut state = state.lock();
		let strategy_config = state.config.strategy.as_ref().unwrap().to_owned();
		let CameraMessage { angle, distance } = state.data.camera_data;
		match state.strategy {
			state::Strategy::Orbit {
				ref mut ball_follow_vector,
				ref mut before_dampen_angle,
				ref mut orbit_angle,
			} => {
				// true angle is needed for the orbit function to work
				let angle = make_bipolar(true_angle(angle));
				let orbit_offset = {
					let config::OrbitConfig {
						curve_steepness,
						shift_x,
						shift_y,
					} = strategy_config.orbit;
					orbit(angle.abs(), curve_steepness, shift_x, shift_y)
				};

				let dampen_amount = {
					let config::DampenConfig {
						curve_steepness,
						shift_x,
						shift_y,
					} = strategy_config.dampen;
					dampen(distance, curve_steepness, shift_x, shift_y)
				};

				let before_dampen = angle + (orbit_offset * angle.signum());
				let after_dampen = angle + (orbit_offset * dampen_amount * angle.signum());

				// true_angle all of the angles because the orbit function uses the true angle plane
				*before_dampen_angle = true_angle(before_dampen);
				*orbit_angle = true_angle(after_dampen);
				*ball_follow_vector = Vec2::from_rad(true_angle(angle)) * distance;

				state.move_vector = Some(*ball_follow_vector);
			}
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
	(PI / 2.0).min(curve_steepness.pow(angle + shift_x) - shift_y)
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
	1.0f64
		.min(curve_steepness.pow(shift_x + distance) - shift_y)
		.max(0.0f64)
}
