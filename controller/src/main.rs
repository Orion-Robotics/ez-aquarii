use anyhow::{Context, Result};
use controller::{
	config::{read_and_watch_config, Config},
	modules::{
		camera, line, motors::Motors, server::StateRecorder, state, state_randomizer, AnyModule,
		Module,
	},
};
use futures::future::join_all;
use parking_lot::Mutex;
use std::{
	sync::Arc,
	time::{Duration, Instant},
};
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

	let robot_state = Arc::new(Mutex::new(state::State::default()));
	let mut modules: Vec<AnyModule> = Vec::new();
	let (mut last_print, mut num_prints) = (Instant::now(), 0);

	loop {
		if last_print.elapsed() >= Duration::from_secs(1) {
			tracing::info!("{} TPS", num_prints);
			last_print = Instant::now();
			num_prints = 0;
		}
		if let Ok(new_config) = cfg_chan.try_recv() {
			match handle_config_change(new_config).await {
				Ok(mut new_modules) => {
					let disabled = modules.iter().map(|x| x.name()).collect::<Vec<_>>();
					let enabled = new_modules.iter().map(|x| x.name()).collect::<Vec<_>>();
					tracing::info!("disabling: {:?}", disabled);
					tracing::info!("enabling: {:?}", enabled);
					for module in modules.iter_mut() {
						module.stop().await.context(module.name())?;
					}
					for module in new_modules.iter_mut() {
						module.start().await.context(module.name())?;
					}
					modules = new_modules;
				}
				Err(e) => tracing::error!("Failed to handle config change: {:#}", e),
			}
		}
		let tick_futures = modules.iter_mut().map(|m| async {
			let mut robot_state = Arc::clone(&robot_state);
			tracing::trace!("{} tick", m.name());
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
		join_all(tick_futures).await;
		num_prints += 1;
	}
}

async fn handle_config_change(cfg: Config) -> Result<Vec<AnyModule>> {
	let Config {
		ref camera,
		ref line,
		ref motors,
		ref server,
		ref state_randomizer,
	} = cfg;

	tracing::info!("config changed, reloading...");
	let mut new_modules: Vec<Box<dyn Module>> = Vec::new();

	if let Some(camera) = camera {
		new_modules.push(Box::new(
			camera::Camera::new(camera.clone())
				.await
				.context("camera creation")?,
		));
	}
	if let Some(line) = line {
		new_modules.push(Box::new(
			line::Line::new(line.clone()).context("line creation")?,
		));
	}
	if let Some(motors) = motors {
		new_modules.push(Box::new(
			Motors::new(motors.clone()).context("motors creation")?,
		));
	}
	if let Some(server) = server {
		new_modules.push(Box::new(
			StateRecorder::new(cfg.clone(), server.clone())
				.await
				.context("server creation")?,
		));
	}
	if *state_randomizer {
		new_modules.push(Box::new(state_randomizer::StateRandomizer::new()));
	}

	Ok(new_modules)
}
