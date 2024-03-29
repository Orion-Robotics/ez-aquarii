use anyhow::Result;

use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use std::{
	f64::consts::E,
	fs::{self, read_to_string},
};

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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default, Copy)]
pub struct ColorThreshold {
	pub blue: u8,
	pub green: u8,
	pub red: u8,
}

impl ColorThreshold {
	pub fn to_array(&self) -> [u8; 3] {
		[self.blue, self.green, self.red]
	}
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default, Copy)]
pub struct ColorRange {
	pub lower: ColorThreshold,
	pub upper: ColorThreshold,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub struct Thresholds {
	pub ball: ColorRange,
	pub yellow: ColorRange,
	pub blue: ColorRange,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub struct CameraConfig {
	pub width: u32,
	pub height: u32,
	pub center_x: f64,
	pub center_y: f64,
	pub framerate: u32,
	pub sensor_mode: u8,
	pub shutter_speed: u32,
	pub balance_red: f32,
	pub balance_blue: f32,
	pub bypass: bool,
	pub saturation: i32,
	pub brightness: u32,
	pub exposure: i32,
	pub iso: i32,
	pub thresholds: Thresholds,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Line {
	pub pickup_threshold: usize,
	pub pickup_sensor_count: usize,
	pub trigger_threshold: usize,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Reader {
	pub uart_path: String,
	pub baud_rate: u32,
	pub line_sensor_count: usize,
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
	/// https://www.desmos.com/calculator/fabhqfn5qz
	pub rotation_slope: f64,
	pub rotation_scalar: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ScoreConditions {
	pub max_distance: f64,
	pub angle_range: f64,
	pub score_goal_angle_range: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Strategy {
	pub orbit: OrbitConfig,
	pub dampen: DampenConfig,
	pub score_conditions: ScoreConditions,
	pub kicker_pin: u8,
}

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub enum Team {
	#[default]
	Yellow,
	Blue,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	pub camera: Option<CameraConfig>,
	pub line: Option<Line>,
	pub server: Option<Server>,
	pub motors: Option<Motors>,
	pub reader: Option<Reader>,
	pub strategy: Option<Strategy>,
	pub state_randomizer: bool,
	pub racing: bool,
	pub team: Team,
	pub software_switch_initialize_pin: u8,
	pub software_switch_toggle_pin: u8,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			software_switch_initialize_pin: 23,
			software_switch_toggle_pin: 24,
			team: Team::Yellow,
			camera: Some(CameraConfig {
				framerate: 90,
				height: 480,
				width: 480,
				center_x: 240.0,
				center_y: 240.0,
				sensor_mode: 7,
				shutter_speed: 15000,
				bypass: false,
				saturation: 0,
				balance_blue: 1.4,
				balance_red: 1.6,
				brightness: 72,
				exposure: 2,
				iso: 500,
				thresholds: Thresholds {
					ball: ColorRange::default(),
					yellow: ColorRange::default(),
					blue: ColorRange::default(),
				},
			}),
			line: Some(Line {
				pickup_threshold: 24,
				pickup_sensor_count: 30,
				trigger_threshold: 400,
			}),
			server: Some(Server {
				addr: "0.0.0.0:7272".to_string(),
			}),
			strategy: Some(Strategy {
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
				score_conditions: ScoreConditions {
					max_distance: 0.5,
					angle_range: 0.5,
					score_goal_angle_range: 0.5,
				},
				kicker_pin: 25,
			}),
			motors: None,
			state_randomizer: false,
			racing: false,
			reader: Some(Reader {
				uart_path: "/dev/ttyAMA0".to_string(),
				line_sensor_count: 46,
				baud_rate: 500000,
			}),
		}
	}
}

impl Config {
	pub async fn save(&self, path: &str) -> Result<()> {
		tokio::fs::write(&path, serde_yaml::to_string(&self)?).await?;
		Ok(())
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
