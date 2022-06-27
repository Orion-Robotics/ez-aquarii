use std::{f64::consts::PI, sync::Arc};

use crate::{
	config::Team,
	modules::state::Strategy::{Orbit, Score, Test},
};
use anyhow::Result;
use async_trait::async_trait;
use futures::{future::select_all, FutureExt};
use num::traits::Pow;
use parking_lot::{Mutex, RwLock};

use crate::{
	config::{self, DampenConfig, OrbitConfig},
	math::{
		angles::{make_bipolar, true_angle},
		vec2::Vec2,
	},
};

use super::{
	state::{self, Blob, ModuleSync, OrbitState, State, TestState},
	Module,
};

pub struct Strategy {}

#[async_trait]
impl Module for Strategy {
	fn name(&self) -> &'static str {
		"strategy"
	}

	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		// sync.reader_notify.notified().await;
		select_all(vec![
			sync.camera_notify.notified().boxed(),
			sync.reader_notify.notified().boxed(),
		])
		.await;
		let mut state = state.write();
		let team = state.config.team;
		let strategy_config = state.config.strategy.as_ref().unwrap().to_owned();
		match state.strategy {
			Orbit(_) => {
				if let Some(line_vector) = state.line_vector {
					state.move_vector = Some(line_vector * -1.0);
					return Ok(());
				}

				if let Some(Blob { angle, distance }) = state.camera_data.ball {
					if distance < strategy_config.score_conditions.max_distance
						&& true_angle(angle).abs() < strategy_config.score_conditions.angle_range
					{
						state.strategy = Score;
					}

					let (before_dampen, after_dampen, ball_location) = eval_orbit(
						angle,
						distance,
						strategy_config.orbit,
						strategy_config.dampen,
					);
					if let state::Strategy::Orbit(ref mut orbit_state) = state.strategy {
						orbit_state.before_dampen_angle = before_dampen;
						orbit_state.orbit_angle = after_dampen;
						orbit_state.ball_follow_vector = ball_location;
					}
					state.move_vector = Some(Vec2::from_rad(after_dampen));
				} else {
					state.move_vector = None;
				}
				if let Some(initial_orientation) = state.initial_orientation {
					state.rotation =
						get_centering_rotation(state.data.orientation, initial_orientation);
				}
			}
			Score => {
				if let Some(Blob { angle, distance }) = state.camera_data.ball {
					if distance > strategy_config.score_conditions.max_distance
						|| true_angle(angle).abs() > strategy_config.score_conditions.angle_range
					{
						state.strategy = Orbit(OrbitState::default());
					}
				}

				if let Some(target_angle) = match team {
					Team::Blue => state.camera_data.blue_goal,
					Team::Yellow => state.camera_data.yellow_goal,
				}
				.map(|goal| goal.angle)
				.or(state.initial_orientation)
				{
					state.rotation = get_centering_rotation(state.data.orientation, target_angle);
				}
			}
			Test(TestState { rotation, vector }) => {
				state.move_vector = Some(vector);
				state.rotation = rotation;
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

pub fn get_centering_rotation(current: f64, target: f64) -> f64 {
	-make_bipolar(((current as f64) - target) % (2.0 * PI))
}

/// eval_orbit
/// Evaluates the orbit motions, taking a angle in radians and trig plane, and the distance to the ball.
pub fn eval_orbit(
	angle: f64,
	distance: f64,
	orbit_config: OrbitConfig,
	dampen_config: DampenConfig,
) -> (f64, f64, Vec2) {
	// true angle is needed for the orbit function to work
	let angle = make_bipolar(true_angle(angle));
	let orbit_offset = {
		let config::OrbitConfig {
			curve_steepness,
			shift_x,
			shift_y,
		} = orbit_config;
		orbit(angle.abs(), curve_steepness, shift_x, shift_y)
	};

	let dampen_amount = {
		let config::DampenConfig {
			curve_steepness,
			shift_x,
			shift_y,
		} = dampen_config;
		dampen(distance, curve_steepness, shift_x, shift_y)
	};

	let before_dampen = angle + (orbit_offset * angle.signum());
	let after_dampen = angle + (orbit_offset * dampen_amount * angle.signum());

	// true_angle all of the angles because the orbit function uses the true angle plane, brings it all
	// back to trig plane.
	(
		true_angle(before_dampen),
		true_angle(after_dampen),
		Vec2::from_rad(true_angle(angle)) * distance,
	)
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
		.min(curve_steepness.pow(shift_x + -distance) - shift_y)
		.max(0.0f64)
}
