use std::collections::HashMap;

use crate::{config::Config, math::vec2::Vec2};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug, Copy)]
pub struct CameraMessage {
	pub angle: f64,
	pub distance: f64,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct RawData {
	pub sensor_data: Vec<u8>,
	pub camera_data: CameraMessage,
}

// State contains all of the robot's data for each tick.
#[derive(Serialize, Default, Clone, Debug)]
pub struct State {
	pub tick_rates: HashMap<String, u32>,

	#[serde(skip_serializing)]
	pub config: Config,
	// raw sensor data, not to be used by actual program logic
	pub data: RawData,
	pub line_detections: Vec<bool>,
	pub line_flipped: bool,
	pub picked_up: bool,
	pub line_vector: Option<Vec2>,
	pub previous_vec: Option<Vec2>,

	pub orbit_offset: f64,
	pub dampen_amount: f64,
	pub orbit_angle: f64,
	pub ball_follow_vector: Vec2,

	pub rotation: f64,
	pub move_vector: Option<Vec2>,
	pub motor_powers: Vec<f32>,
}

impl State {
	/// convenience function to print the entire state as a json string
	pub fn print_state(&self) {
		println!("{}", serde_json::to_string(&self).unwrap());
	}
}
