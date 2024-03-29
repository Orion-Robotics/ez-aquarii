use anyhow::{Context, Result};
use controller::{
	config::{read_config, Config},
	modules::{
		camera, line,
		motors::Motors,
		racing::Racing,
		reader::Reader,
		server::StateRecorder,
		state::{self, ModuleSync},
		state_randomizer,
		strategy::Strategy,
		AnyModule, Module,
	},
};
use futures::future::join_all;
use parking_lot::{Mutex, RwLock};
use rppal::gpio::{Gpio, Level, Trigger};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::interval;
use tracing_subscriber::EnvFilter;

pub const CONFIG_FILE: &str = "./config.yaml";

#[tokio::main]
async fn main() -> Result<()> {
	// sets the debug level to show all traces
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	let config = read_config(CONFIG_FILE).await?;
	let robot_state = Arc::new(RwLock::new(state::State::default()));
	let module_sync = ModuleSync::default();
	let gpio = Gpio::new()?;
	let mut initialize_pin = gpio
		.get(config.software_switch_initialize_pin)?
		.into_input();
	let mut toggle_pin = gpio.get(config.software_switch_toggle_pin)?.into_input();
	{
		let robot_state = Arc::clone(&robot_state);
		initialize_pin.set_async_interrupt(Trigger::RisingEdge, move |_| {
			robot_state.write().initial_orientation = None;
		})?;
	}
	{
		let robot_state = Arc::clone(&robot_state);
		toggle_pin.set_async_interrupt(Trigger::Both, move |lvl| {
			robot_state.write().paused = lvl == Level::High;
		})?;
	}

	let mut modules: Vec<AnyModule> = handle_config_change(config.clone()).await?;
	for module in modules.iter_mut() {
		module.start().await?;
	}
	robot_state.write().config = config.clone();
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
		robot_state.write().tick_rates = pre_tick_rates.clone();
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
		ref racing,
		ref reader,
		ref strategy,
		..
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
	if let Some(reader) = reader {
		new_modules.push(Box::new(
			Reader::new(reader.clone())
				.await
				.context("reader creation")?,
		));
	}
	if let Some(strategy) = strategy {
		new_modules.push(Box::new(
			Strategy::new(strategy.clone()).context("strategy creation")?,
		));
	}
	if *state_randomizer {
		new_modules.push(Box::new(state_randomizer::StateRandomizer::new()));
	}

	if *racing {
		new_modules.push(Box::new(Racing {}));
	}

	Ok(new_modules)
}
