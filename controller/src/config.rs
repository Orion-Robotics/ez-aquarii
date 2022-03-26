use anyhow::Result;
use async_recursion::async_recursion;
use notify::{Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashSet,
	fs::{self, read_to_string},
	path::{Path, PathBuf},
};
use tokio::sync::mpsc;

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Module {
	Camera {
		path: PathBuf,
	},
	Line {
		sensor_count: usize,
		pickup_threshold: usize,
		uart_path: String,
		baud_rate: u32,
	},
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct StateHistory {
	pub enable: bool,
	pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	pub modules: HashSet<Module>,
	pub state_history: StateHistory,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			modules: HashSet::from([
				Module::Camera {
					path: PathBuf::from("./socket"),
				},
				Module::Line {
					sensor_count: 46,
					pickup_threshold: 24,
					uart_path: "/dev/ttyUSB0".to_string(),
					baud_rate: 500000,
				},
			]),
			state_history: StateHistory {
				enable: false,
				path: PathBuf::from("./state.json"),
			},
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
