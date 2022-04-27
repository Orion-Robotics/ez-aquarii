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
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Module {
	Camera(Camera),
	Line(Line),
	StateRandomizer,
	Server(Server),
	Motors(Motors),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	pub modules: Vec<Module>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			modules: Vec::from([
				Module::Camera(Camera {
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
				Module::Line(Line {
					sensor_count: 46,
					pickup_threshold: 24,
					pickup_sensor_count: 30,
					trigger_threshold: 400,
					uart_path: "/dev/ttyUSB0".to_string(),
					baud_rate: 500000,
				}),
				Module::Server(Server {
					addr: "0.0.0.0:7272".to_string(),
				}),
			]),
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

// read_and_watch_config reads the config file and watches it for changes.
// If the config file changes, it will be read again and the new config will be sent over the channel.
// If there is an error, the error will be sent over the other channel.
// It is important to hold onto the INotifyWatcher so that it doesn't get dropped and die.
pub async fn read_and_watch_config(
	path: &'static str,
) -> Result<(INotifyWatcher, mpsc::UnboundedReceiver<Config>)> {
	let (s, r) = mpsc::unbounded_channel();
	let _ = s.send(read_config(path).await?);
	let mut watcher = RecommendedWatcher::new(move |res: notify::Result<Event>| match res {
		Ok(event) => {
			if !event.kind.is_modify() {
				return;
			}
			if let Ok(new_config) = futures::executor::block_on(read_config(path)) {
				if let Err(err) = s.send(new_config) {
					tracing::error!("failed to send config: {}", err);
				}
			}
		}
		Err(err) => {
			tracing::error!("failed to watch config {:?}", err);
		}
	})?;
	watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;
	Ok((watcher, r))
}
