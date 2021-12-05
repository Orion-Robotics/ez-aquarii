use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Module {
    Camera { path: PathBuf },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub modules: HashSet<Module>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modules: HashSet::from([Module::Camera {
                path: PathBuf::from("./socket"),
            }]),
        }
    }
}
