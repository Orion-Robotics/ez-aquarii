use anyhow::Result;

use async_recursion::async_recursion;
use notify::{Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
	f64::consts::E,
	fs::{self, read_to_string},
	path::{Path, PathBuf},
};
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OrbitConfig {
	pub curve_steepness: f64,
	pub shift_x: f64,
	pub shift_y: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DampenConfig {
	pub curve_steepness: f64,
	pub shift_x: f64,
	pub shift_y: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Camera {
	pub enable_reading: bool,
	pub path: PathBuf,
	pub orbit: OrbitConfig,
	pub dampen: DampenConfig,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Line {
	pub sensor_count: usize,
	pub pickup_threshold: usize,
	pub pickup_sensor_count: usize,
	pub trigger_threshold: usize,
	pub uart_path: String,
	pub baud_rate: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Server {
	pub addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Motors {
	pub uart_path: String,
	pub baud_rate: u32,
	pub motor_offset: f64,
	pub speed: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	pub camera: Option<Camera>,
	pub line: Option<Line>,
	pub server: Option<Server>,
	pub motors: Option<Motors>,
	pub state_randomizer: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			camera: Some(Camera {
				enable_reading: true,
				path: PathBuf::from("./socket"),
				orbit: OrbitConfig {
					curve_steepness: E,
					shift_x: 0.3,
					shift_y: 1.0,
				},
				dampen: DampenConfig {
					curve_steepness: 305.0,
					shift_x: -1.0,
					shift_y: 0.0,
				},
			}),
			line: Some(Line {
				sensor_count: 46,
				pickup_threshold: 24,
				pickup_sensor_count: 30,
				trigger_threshold: 400,
				uart_path: "/dev/ttyACM0".to_string(),
				baud_rate: 500000,
			}),
			server: Some(Server {
				addr: "0.0.0.0:7272".to_string(),
			}),
			motors: None,
			state_randomizer: false,
		}
	}
}

#[async_recursion]
pub async fn read_config(path: &str) -> Result<Config> {
	match read_to_string(path) {
		Ok(config) => Ok(serde_yaml::from_str::<Config>(&config)?),
		Err(_) => {
			fs::write(&path, serde_yaml::to_string(&Config::default())?)?;
			read_config(path).await
		}
	}
}
