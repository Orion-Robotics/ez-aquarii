use anyhow::{Context, Result};
use controller::comms;
use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::OpenOptionsExt,
    path::PathBuf,
};

use crate::ipc;

use super::Module;

pub struct Camera {
    pub socket_file: File,
}

impl Camera {
    pub fn new(path: PathBuf) -> Result<Camera> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .open(path)
            .with_context(|| "failed to open file")?;
        Ok(Camera { socket_file: f })
    }
}

impl Module for Camera {
    fn tick(&self) -> Result<()> {
        let data = ipc::read_proto::<comms::Packet, _>(&self.socket_file)
            .with_context(|| "failed to read packet")?;
        println!("{:?}", data.time);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "camera"
    }
}
