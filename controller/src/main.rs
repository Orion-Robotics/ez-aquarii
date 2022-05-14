use anyhow::{Context, Result};
use controller::{
	config::{read_config, Config},
	modules::{
		camera, line, motors::Motors, server::StateRecorder, state, state_randomizer, AnyModule,
		Module,
	},
};
use futures::future::join_all;
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::interval;
use tracing_subscriber::EnvFilter;

const CONFIG_FILE: &str = "./config.yaml";

#[tokio::main]
async fn main() -> Result<()> {
	// sets the debug level to show all traces
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	let config = read_config(CONFIG_FILE).await?;
	let mut modules: Vec<AnyModule> = handle_config_change(config.clone()).await?;
	for module in modules.iter_mut() {
		module.start().await?;
	}
	let robot_state = Arc::new(Mutex::new(state::State::default()));
	robot_state.lock().config = config.clone();
	let pre_tick_rates: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
	let futures = modules
		.into_iter()
		.enumerate()
		.map(|(_, mut m)| {
			let mut state = robot_state.clone();
			let pre_tick_rates = pre_tick_rates.clone();
			tokio::spawn(async move {
				loop {
					tracing::debug!("Ticking module {:?}", m.name());
					if let Err(e) = m.tick(&mut state).await {
						tracing::error!("error ticking {}: {:?}", m.name(), e);
					}
					pre_tick_rates
						.lock()
						.entry(m.name().to_string())
						.and_modify(|v| *v += 1)
						.or_insert(0);
				}
			})
		})
		.collect::<Vec<_>>();

	tokio::spawn(join_all(futures));

	let mut interval = interval(Duration::from_millis(1000));
	loop {
		interval.tick().await;
		let mut pre_tick_rates = pre_tick_rates.lock();
		robot_state.lock().tick_rates = pre_tick_rates.clone();
		let mut formatted = pre_tick_rates.iter().collect::<Vec<_>>();
		formatted.sort_by(|(name, _), (name1, _)| name1.cmp(name));
		tracing::info!("tick rates: {:?}", formatted);
		pre_tick_rates.clear();
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
