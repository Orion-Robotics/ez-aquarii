use anyhow::Result;
use async_trait::async_trait;

use crate::config::Config;

use self::state::State;

pub mod camera;
pub mod line;
#[cfg(test)]
pub mod line_test;
pub mod motors;
pub mod server;
pub mod state;
pub mod state_randomizer;
pub mod strategy;

#[async_trait]
pub trait Module: Send {
	fn name(&self) -> &'static str;
	async fn tick(&mut self, state: &mut State) -> Result<()>;
	async fn start(&mut self) -> Result<()>;
	async fn stop(&mut self) -> Result<()>;
}

pub type AnyModule = Box<dyn Module>;
