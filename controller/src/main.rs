use anyhow::{Context, Result};
use controller::{
	config,
	config::{read_and_watch_config, Config},
	modules::{
		camera, line, motors::Motors, server::StateRecorder, state, state::State, state_randomizer,
		AnyModule, Module,
	},
};
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};
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
			match handle_config_change(new_config).await {
				Ok(mut new_modules) => {
					let disabled = modules.iter().map(|x| x.name()).collect::<Vec<_>>();
					let enabled = new_modules.iter().map(|x| x.name()).collect::<Vec<_>>();
					tracing::info!("disabling: {:?}", disabled);
					tracing::info!("enabling: {:?}", enabled);
					for module in modules.iter_mut() {
						module.stop().await?;
					}
					for module in new_modules.iter_mut() {
						module.start().await?;
					}
					modules = new_modules;
				}
				Err(e) => tracing::error!("Failed to handle config change: {}", e),
			}
		}
		tick_modules(&mut modules, &mut robot_state).await;
	}
}

async fn tick_modules(modules: &mut [AnyModule], robot_state: &mut Arc<Mutex<State>>) {
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

async fn handle_config_change(new_config: Config) -> Result<Vec<AnyModule>> {
	tracing::info!("config changed, reloading...");
	let module_configs = &new_config.modules;

	let mut new_modules: Vec<Box<dyn Module>> = Vec::new();
	for m in module_configs {
		let module_instance: AnyModule = match m {
			config::Module::StateRandomizer => Box::new(state_randomizer::StateRandomizer {}),
			config::Module::Camera(cfg) => Box::new(camera::Camera::new(cfg.clone()).await?),
			config::Module::Line(cfg) => Box::new(line::Line::new(cfg.clone())?),
			config::Module::Server(cfg) => {
				Box::new(StateRecorder::new(new_config.clone(), cfg.clone()).await?)
			}
			config::Module::Motors(cfg) => Box::new(Motors::new(cfg.clone()).await?),
		};
		new_modules.push(module_instance);
	}

	Ok(new_modules)
}
