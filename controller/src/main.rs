use anyhow::{Context, Result};
use controller::{
	config::{read_config, Config},
	modules::{
		camera, line,
		motors::Motors,
		reader::Reader,
		server::StateRecorder,
		state::{self, ModuleSync},
		state_randomizer,
		strategy::Strategy,
		AnyModule, Module,
	},
};
use futures::future::join_all;
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Notify, time::interval};
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
	let module_sync = ModuleSync::default();
	robot_state.lock().config = config.clone();
	let pre_tick_rates: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
	let futures = modules
		.into_iter()
		.enumerate()
		.map(|(_, mut m)| {
			let mut state = robot_state.clone();
			let pre_tick_rates = pre_tick_rates.clone();
			let mut module_sync = module_sync.clone();
			tokio::spawn(async move {
				loop {
					if let Err(e) = m.tick(&mut state, &mut module_sync).await {
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
		ref reader,
		ref strategy,
		..
	} = cfg;

	let mut new_modules: Vec<Box<dyn Module>> = Vec::new();

	if let Some(camera) = camera {
		new_modules.push(Box::new(
			camera::Camera::new(camera)
				.await
				.context("camera creation")?,
		));
	}
	if let Some(line) = line {
		new_modules.push(Box::new(
			line::Line::new(line).context("line creation")?,
		));
	}
	if let Some(motors) = motors {
		new_modules.push(Box::new(
			Motors::new(motors).context("motors creation")?,
		));
	}
	if let Some(server) = server {
		new_modules.push(Box::new(
			StateRecorder::new(cfg, server)
				.await
				.context("server creation")?,
		));
	}
	if let Some(reader) = reader {
		new_modules.push(Box::new(
			Reader::new(reader)
				.await
				.context("reader creation")?,
		));
	}
	if strategy.is_some() {
		new_modules.push(Box::new(Strategy {}));
	}
	if *state_randomizer {
		new_modules.push(Box::new(state_randomizer::StateRandomizer::new()));
	}

	Ok(new_modules)
}
