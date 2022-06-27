use std::sync::Arc;

use crate::{
	config::{self},
	modules,
};
use anyhow::Result;
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::Mutex;
use std::slice;
use std::thread;
use std::time::Duration;
use tokio::fs::{File, OpenOptions};
// use std::fs::OpenOptions;
// use std::io::Write;
use opencv::core::{Mat, CV_8SC3};
use opencv::highgui;
use std::ffi::c_void;

use super::{
	state::{ModuleSync, State},
	Module,
};

pub struct Camera {}

impl Camera {
	pub async fn new() -> Result<Self> {
		let (w, h) = (720, 720);
		ffi::initialize_camera(w, h);
		thread::sleep(Duration::from_secs(3));
		Ok(Camera {})
	}
}
#[cxx::bridge]
mod ffi {
	struct ImagePacket {
		data: *mut u8,
		len: usize,
	}

	unsafe extern "C++" {
		include!("nffi/include/imageprovider.h");
		include!("raspicam/raspicam.h");
		// include!("/usr/include/opencv4/opencv2/");

		type Cam;
		type ImagePacket;

		fn get_image_packet() -> ImagePacket;
		fn initialize_camera(w: u32, h: u32) -> ();
		// fn get_image(cam: UniquePtr<Cam>) -> *mut u8;
	}
}
#[async_trait]
impl Module for Camera {
	fn name(&self) -> &'static str {
		"camera"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>, sync: &mut ModuleSync) -> Result<()> {
		let pkt = ffi::get_image_packet();
		let imslice = unsafe { slice::from_raw_parts_mut(pkt.data, pkt.len) };
		let mat = unsafe {
			Mat::new_nd_with_data(
				&[w as i32, h as i32],
				CV_8SC3,
				imslice.as_mut_ptr() as *mut c_void,
				Some(&[1]),
			)?
		};

		let mut state = state.lock();
		state.camera_data.ball = get_location(0);
		state.camera_data.yellow_goal = get_location(1);
		state.camera_data.blue_goal = get_location(2);
		state.data.camera_data = data;
		sync.camera_notify.notify_waiters();
		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
