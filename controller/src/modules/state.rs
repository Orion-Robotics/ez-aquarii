use crate::math::vec2::Vec2;
use serde::Serialize;

#[derive(Serialize, Default, Clone, Debug)]
pub struct RawData {
	pub sensor_data: Vec<u8>,
}

// State contains all of the robot's data for each tick.
#[derive(Serialize, Default, Clone, Debug)]
pub struct State {
	// raw sensor data, not to be used by actual program logic
	pub data: RawData,
	pub line_detections: Vec<bool>,
	pub line_flipped: bool,
	pub line_vector: Vec2,
	pub move_vector: Vec2,
}

impl State {
	// convenience function to print the entire state as a json string
	pub fn print_state(&self) -> () {
		println!("{}", serde_json::to_string(&self).unwrap());
	}
}
