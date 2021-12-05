use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Module {
    Camera,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub modules: Vec<Module>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modules: vec![Module::Camera],
        }
    }
}
