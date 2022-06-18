pub mod cv;
use crate::{config, ffi};
use anyhow::Result;
use async_trait::async_trait;
use cv::{ball_heuristic, find_best_contour};
use opencv::{
	core::CV_8UC3,
	prelude::{Mat, MatTraitConstManual},
};
use parking_lot::Mutex;
use std::{ffi::c_void, slice, sync::Arc, time::Duration};
// use std::fs::OpenOptions;
// use std::io::Write;

use super::{
	state::{ModuleSync, State},
	Module,
};

pub struct Camera {
	cfg: config::Camera,
}

impl Camera {
	pub async fn new(cfg: config::Camera) -> Result<Self> {
		unsafe {
			ffi::initialize_camera(
				cfg.width,
				cfg.height,
				cfg.framerate,
				cfg.sensor_mode,
				cfg.shutter_speed,
			);
		}
		tokio::time::sleep(Duration::from_secs(3)).await;
		Ok(Camera { cfg })
	}
}

#[async_trait]
impl Module for Camera {
	fn name(&self) -> &'static str {
		"camera"
	}

	async fn tick(&mut self, state: &mut Arc<Mutex<State>>, sync: &mut ModuleSync) -> Result<()> {
		let mut state = state.lock();
		let pkt = unsafe { ffi::get_image_packet() };
		let mut imslice = unsafe { slice::from_raw_parts_mut(pkt.data, pkt.len) };
		let mut mat = unsafe {
			Mat::new_nd_with_data(
				&[self.cfg.width as i32, self.cfg.height as i32],
				CV_8UC3,
				imslice.as_mut_ptr() as *mut c_void,
				None,
			)?
		};
		let ball = find_best_contour(
			&mat,
			[0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0],
			ball_heuristic(0.5, 0.5),
		)?;
		let yellow_goal = find_best_contour(
			&mat,
			[0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0],
			ball_heuristic(1.0, 0.5),
		)?;
		let blue_goal = find_best_contour(
			&mat,
			[0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0],
			ball_heuristic(1.0, 0.5),
		)?;
		tracing::debug!(
			"Data: {:?} {:?}, {:?}, {:?}",
			mat.size()?,
			ball,
			yellow_goal,
			blue_goal
		);
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
