use anyhow::{Context, Result};
use controller::{
	config,
	config::{read_and_watch_config, Config},
	modules::{
		camera, line, motors::Motors, state, state::State, state_randomizer,
		state_recorder::StateRecorder, AnyModule, Module,
	},
};
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};
use tokio;
use tracing::Instrument;
use tracing_subscriber::EnvFilter;

const CONFIG_FILE: &str = "./config.yaml";

#[tokio::main]
async fn main() -> Result<()> {
	// sets the debug level to show all traces
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
		)
		.init();

	let (_watcher, mut cfg_chan) = read_and_watch_config(CONFIG_FILE)
		.instrument(tracing::info_span!("Reading config file..."))
		.await
		.with_context(|| format!("Failed to read config file {CONFIG_FILE}"))?;

	let mut robot_state = Arc::new(Mutex::new(state::State::default()));
	let mut modules: Vec<AnyModule> = Vec::new();

	loop {
		if let Ok(new_config) = cfg_chan.try_recv() {
			let new_modules = handle_config_change(new_config).await;

			for module in modules.iter_mut() {
				module.stop().await?;
			}

			modules = new_modules;

			for module in modules.iter_mut() {
				module.start().await?;
			}
		}
		tick_modules(&mut modules, &mut robot_state).await;
	}
}

async fn tick_modules(modules: &mut Vec<AnyModule>, robot_state: &mut Arc<Mutex<State>>) {
	let tick_futures = modules.iter_mut().map(|m| async {
		tracing::trace!("{} tick", m.name());
		let mut robot_state = robot_state.lock();
		let tick_future =
			tokio::time::timeout(Duration::from_millis(200), m.tick(&mut robot_state));
		match tick_future.await {
			Ok(res) => {
				if let Err(e) = res {
					tracing::error!("{} encountered an error while ticking: {:?}", m.name(), e);
				}
			}
			Err(e) => tracing::warn!("{} took too long to process: {}", m.name(), e),
		}
	});
	// TODO: get this working with join_all for concurrent processing.
	for future in tick_futures {
		future.await;
	}
}

async fn handle_config_change(new_config: Config) -> Vec<AnyModule> {
	tracing::info!("config changed, reloading...");
	let module_configs = &new_config.modules;

	let mut new_modules: Vec<Box<dyn Module>> = Vec::new();
	for m in module_configs {
		let module_instance: AnyModule = match m {
			config::Module::StateRandomizer => Box::new(state_randomizer::StateRandomizer {}),
			config::Module::Camera { path } => {
				Box::new(camera::Camera::new(path.clone()).await.unwrap())
			}
			config::Module::Line {
				trigger_threshold,
				pickup_threshold,
				sensor_count,
				baud_rate,
				uart_path,
			} => Box::new(
				line::Line::new(
					uart_path.to_string(),
					*baud_rate,
					*trigger_threshold,
					*pickup_threshold,
					*sensor_count,
				)
				.unwrap(),
			),
			config::Module::Server { addr } => Box::new(
				StateRecorder::new(new_config.clone(), addr.clone())
					.await
					.unwrap(),
			),
			config::Module::Motors {
				uart_path,
				baud_rate,
				motor_offset,
			} => Box::new(
				Motors::new(uart_path.to_string(), *baud_rate, *motor_offset)
					.await
					.unwrap(),
			),
		};
		new_modules.push(module_instance);
	}

	new_modules
}
