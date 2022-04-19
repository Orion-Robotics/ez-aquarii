use anyhow::Result;
use async_recursion::async_recursion;
use notify::{Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
	fs::{self, read_to_string},
	path::{Path, PathBuf},
};
use tokio::sync::mpsc;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Module {
	Camera {
		path: PathBuf,
	},
	Line {
		sensor_count: usize,
		pickup_threshold: usize,
		pickup_sensor_count: usize,
		trigger_threshold: usize,
		uart_path: String,
		baud_rate: u32,
	},
	StateRandomizer,
	Server {
		addr: String,
	},
	Motors {
		uart_path: String,
		baud_rate: u32,
		motor_offset: f64,
	},
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	pub modules: Vec<Module>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			modules: Vec::from([
				Module::Camera {
					path: PathBuf::from("./socket"),
				},
				Module::Line {
					sensor_count: 46,
					pickup_threshold: 24,
					pickup_sensor_count: 30,
					trigger_threshold: 400,
					uart_path: "/dev/ttyUSB0".to_string(),
					baud_rate: 500000,
				},
				Module::Server {
					addr: "0.0.0.0:7272".to_string(),
				},
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
