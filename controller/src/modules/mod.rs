use anyhow::Result;

pub mod camera;

pub trait Module {
    fn name(&self) -> &'static str;
    fn tick(&self) -> Result<()>;
}
