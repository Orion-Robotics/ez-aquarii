use anyhow::Result;
use controller::comms;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    os::unix::prelude::OpenOptionsExt,
};

use crate::ipc;

use super::Module;

pub struct Camera {
    pub socket_file: File,
}

impl Camera {
    fn new(path: &str) -> Result<Camera, Box<dyn Error>> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)?;
        Ok(Camera { socket_file: f })
    }
}

impl Module for Camera {
    fn tick(&self) -> Result<()> {
        let data = ipc::read_proto::<comms::Packet, _>(&self.socket_file)?;
        println!("{:?}", data.time);
        Ok(())
    }
}
