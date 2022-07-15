use std::sync::Arc;

use crate::math::vec2::Vec2;

use super::{
	state::{ModuleSync, State},
	Module,
};
use anyhow::Result;
use async_trait::async_trait;
use parking_lot::RwLock;

pub struct Racing {}

#[async_trait]
impl Module for Racing {
	fn name(&self) -> &'static str {
		"racing"
	}

	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		state.write().move_vector = Some((Vec2::new(0.0, 1.0), false));
		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
