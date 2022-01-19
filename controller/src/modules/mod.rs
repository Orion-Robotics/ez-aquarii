use anyhow::Result;
use async_trait::async_trait;

use self::state::State;

pub mod camera;
pub mod line;
pub mod state;

#[async_trait]
pub trait Module: Send {
    fn name(&self) -> &'static str;
    async fn tick(&mut self, state: &mut State) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
}

pub type AnyModule = Box<dyn Module>;
