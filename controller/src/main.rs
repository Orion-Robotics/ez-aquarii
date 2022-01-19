use anyhow::Context;
use anyhow::Result;
use config::read_and_watch_config;
use config::Config;
use futures::future::join_all;
use modules::state;
use modules::state::State;
use modules::AnyModule;
use parking_lot::Mutex;
use recorder::Recorder;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tracing::Instrument;
use tracing::Level;

use crate::modules::camera;
use crate::modules::line;
use crate::modules::Module;

pub mod config;
pub mod ipc;
pub mod math;
pub mod modules;
pub mod recorder;

const CONFIG_FILE: &str = "./config.yaml";

#[tokio::main]
async fn main() -> Result<()> {
	// sets the debug level to show all traces
	tracing_subscriber::fmt()
		.with_max_level(Level::TRACE)
		.init();

	let (_watcher, mut cfg_chan) = read_and_watch_config(CONFIG_FILE)
		.instrument(tracing::info_span!("Reading config file..."))
		.await
		.with_context(|| format!("Failed to read config file {CONFIG_FILE}"))?;

	let mut robot_state = Arc::new(Mutex::new(state::State::default()));
	let mut modules: Vec<AnyModule> = Vec::new();
	let history_file: Arc<Mutex<Option<std::fs::File>>> = Arc::new(Mutex::new(None));

	let recorder: Recorder<_, State> = Recorder::new(history_file.clone());

	loop {
		if let Ok(new_config) = cfg_chan.try_recv() {
			let (new_modules, new_history_file) = handle_config_change(new_config).await;

			for module in modules.iter_mut() {
				module.stop().await?;
			}

			modules = new_modules;
			*history_file.lock() = new_history_file;

			for module in modules.iter_mut() {
				module.start()?;
			}
		}
		tick_modules(&mut modules, &mut robot_state).await;
		let robot_state = &*robot_state.lock();
		let _ = recorder.record(robot_state.clone());
	}
}

async fn tick_modules(modules: &mut Vec<AnyModule>, robot_state: &mut Arc<Mutex<State>>) {
	let tick_futures = modules.iter_mut().map(|m| async {
		tracing::debug!("{} tick", m.name());
		let mut robot_state_copy = robot_state.lock().clone();
		let tick_future =
			tokio::time::timeout(Duration::from_millis(200), m.tick(&mut robot_state_copy));
		match tick_future.await {
			Ok(res) => {
				if let Err(e) = res {
					tracing::error!("{} encountered an error while ticking: {:?}", m.name(), e);
				}
				*robot_state.lock() = robot_state_copy;
			}
			Err(e) => tracing::warn!("{} took too long to process: {}", m.name(), e),
		}
	});
	join_all(tick_futures).await;
}

async fn handle_config_change(new_config: Config) -> (Vec<AnyModule>, Option<std::fs::File>) {
	tracing::info!("config changed, reloading...");
	let module_configs = &new_config.modules;

	let mut new_modules: Vec<Box<dyn Module>> = Vec::new();
	for m in module_configs {
		let module_instance: AnyModule = match m {
			config::Module::Camera { path } => {
				Box::new(camera::Camera::new(path.clone()).await.unwrap())
			}
			config::Module::Line { .. } => Box::new(line::Line::new()),
		};
		new_modules.push(module_instance);
	}

	let mut history_file = None;

	if new_config.state_history.enable {
		history_file = Some(
			std::fs::OpenOptions::new()
				.write(true)
				.create(true)
				.append(true)
				.open(&new_config.state_history.path)
				.unwrap(),
		);
	}

	(new_modules, history_file)
}
