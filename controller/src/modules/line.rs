use anyhow::Result;
use async_trait::async_trait;

use super::{state::State, Module};

pub struct Line {}

#[async_trait]
impl Module for Line {
    async fn tick(&mut self, state: &mut State) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "line"
    }

    fn start(&mut self) -> Result<()> {
        todo!()
    }

    async fn stop(&mut self) -> Result<()> {
        todo!()
    }
}
