use crate::math::vec2::Vec2;
use arrayvec::ArrayVec;
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
pub struct RawData {
	sensor_data: Vec<u8>,
}

// State contains all of the robot's data for each tick.
#[derive(Serialize, Default, Clone)]
pub struct State {
	// raw sensor data, not to be used by actual program logic
	pub data: RawData,
	pub line_detections: Vec<bool>,
	pub line_flipped: bool,
	pub line_vector: Vec2,
	pub move_vector: Vec2,
}
