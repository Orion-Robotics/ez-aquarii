use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::Mutex;
use rand::Rng;

use crate::math::vec2::Vec2;

use super::{state::State, Module};

pub struct StateRandomizer {}

impl StateRandomizer {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for StateRandomizer {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl Module for StateRandomizer {
	fn name(&self) -> &'static str {
		"state_randomizer"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>) -> anyhow::Result<()> {
		let mut rng = rand::thread_rng();
		let mut state = state.lock();

		state.data.sensor_data = vec![0; 46].iter().map(|_| rng.gen::<u8>()).collect();

		state.line_flipped = !state.line_flipped;
		state.line_detections = state.data.sensor_data.iter().map(|&v| v > 128).collect();

		let mut random_vec = || {
			Vec2 {
				x: rng.gen_range(-1.0..1.0),
				y: rng.gen_range(-1.0..1.0),
			}
			.normalize()
		};

		state.line_vector = Some(random_vec());
		state.move_vector = Some(random_vec());

		Ok(())
	}

	async fn start(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> anyhow::Result<()> {
		Ok(())
	}
}
