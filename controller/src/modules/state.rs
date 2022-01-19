use arrayvec::ArrayVec;
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
pub struct RawData {
    sensor_data: ArrayVec<u8, 46>,
}

// State contains all of the robot's data for each tick.
#[derive(Serialize, Default, Clone)]
pub struct State {
    pub data: RawData,
    pub line_vec: (f32, f32),
}
