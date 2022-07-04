use std::{collections::HashMap, sync::Arc};

use crate::{config::Config, math::vec2::Vec2};
use opencv::prelude::Mat;
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use tokio::sync::{watch, Notify};

#[derive(Debug, Clone, Default)]
pub struct ModuleSync {
	pub reader_notify: Arc<Notify>,
	pub camera_notify: Arc<Notify>,
	pub frame: Arc<Mutex<Mat>>,
}

// impl Default for ModuleSync {
// 	fn default() -> Self {
// 		Self {
// 			reader_notify: Default::default(),
// 			camera_notify: Default::default(),
// 			frame: Arc::new(Mutex::new(Mat::default())),
// 		}
// 	}
// }

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default)]
pub struct OrbitState {
	pub before_dampen_angle: f64,
	pub orbit_angle: f64,
	pub ball_follow_vector: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct TestState {
	pub rotation: f64,
	pub vector: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(tag = "type")]
pub enum Strategy {
	Orbit(OrbitState),
	Score,
	Test(TestState),
}

impl Default for Strategy {
	fn default() -> Self {
		Strategy::Orbit(OrbitState {
			before_dampen_angle: 0.0,
			orbit_angle: 0.0,
			ball_follow_vector: Vec2::new(0.0, 0.0),
		})
	}
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, Copy)]
pub struct Blob {
	pub angle: f64,
	pub distance: f64,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct RawData {
	pub sensor_data: Vec<u8>,
	pub orientation: f64,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct CameraData {
	pub yellow_goal: Option<Blob>,
	pub blue_goal: Option<Blob>,
	pub ball: Option<Blob>,
}

// State contains all of the robot's data for each tick.
#[derive(Serialize, Default, Clone, Debug)]
pub struct State {
	pub tick_rates: HashMap<String, u32>,

	#[serde(skip_serializing)]
	pub config: Config,

	// raw sensor data, not to be used by actual program logic
	pub data: RawData,
	pub camera_data: CameraData,
	pub line_detections: Vec<bool>,
	pub line_flipped: bool,
	pub picked_up: bool,
	pub line_vector: Option<Vec2>,
	pub previous_vec: Option<Vec2>,

	pub strategy: Strategy,

	pub initial_orientation: Option<f64>,
	pub rotation: f64,
	pub before_line_vector: Option<Vec2>,

	pub move_vector: Option<Vec2>,
	pub motor_powers: Vec<f32>,
}

impl State {
	/// convenience function to print the entire state as a json string
	pub fn print_state(&self) {
		println!("{}", serde_json::to_string(&self).unwrap());
	}
}
