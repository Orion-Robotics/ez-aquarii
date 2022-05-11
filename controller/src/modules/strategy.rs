use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use parking_lot::Mutex;

use super::{state::State, Module};

pub struct Strategy {}

#[async_trait]
impl Module for Strategy {
	fn name(&self) -> &'static str {
		"strategy"
	}

	async fn tick(&mut self, _: &mut Arc<Mutex<State>>) -> Result<()> {
		todo!()
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
