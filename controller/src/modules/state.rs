use crate::math::vec2::Vec2;
use arrayvec::ArrayVec;
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
pub struct RawData {
	sensor_data: ArrayVec<u8, 46>,
}

// State contains all of the robot's data for each tick.
#[derive(Serialize, Default, Clone)]
pub struct State {
	// raw sensor data, not to be used by actual program logic
	pub data: RawData,
	// line_vec, the current vector pointing towards the field at all times.
	pub line_vec: Vec2,
}
