use anyhow::Result;

pub mod camera;

pub trait Module {
    fn tick(&self) -> Result<()>;
}
