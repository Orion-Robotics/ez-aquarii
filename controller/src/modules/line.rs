use anyhow::Result;
use async_trait::async_trait;

use crate::math::vec2::{angle_between, Vec2};

use super::{state::State, Module};

pub struct Line {}

impl Line {
	pub fn new() -> Self {
		Line {}
	}
}

#[async_trait]
impl Module for Line {
	async fn tick(&mut self, state: &mut State) -> Result<()> {
		Ok(())
	}

	fn name(&self) -> &'static str {
		"line"
	}

	fn start(&mut self) -> Result<()> {
		Ok(())
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
	module.tick(&mut state).await?;
	Ok(())
}
