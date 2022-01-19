use anyhow::Result;
use async_trait::async_trait;

use crate::math::vec2::{angle_between, Vec2};

use super::{state::State, Module};

pub struct Line {}

#[async_trait]
impl Module for Line {
    async fn tick(&mut self, state: &mut State) -> Result<()> {
        let current_vec = state.raw_line_vec;
        let prev_vec = state.prev_line_vec;

        let diff = angle_between(current_vec, prev_vec);

        tracing::debug!("Angle difference: {:?}", diff);
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

// Test that the line vector will flip if it crosses over
#[tokio::test]
async fn test_line_flip() -> Result<()> {
    let mut module = Line {};
    let mut state = State::default();
    state.raw_line_vec = Vec2 { x: 1.0, y: 0.0 };
    state.prev_line_vec = Vec2 { x: -1.0, y: 0.0 };
    module.tick(&mut state).await?;
    Ok(())
}
