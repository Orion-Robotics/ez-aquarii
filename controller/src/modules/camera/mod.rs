pub mod cv;
use crate::{config, ffi};
use anyhow::Result;
use async_trait::async_trait;
use cv::{ball_heuristic, find_best_contour};
use opencv::{
	core::CV_8UC3,
	prelude::{Mat, MatTraitConstManual},
};
use parking_lot::RwLock;
use std::{ffi::c_void, slice, sync::Arc, time::Duration};
use tokio::sync::mpsc;
// use std::fs::OpenOptions;
// use std::io::Write;

use super::{
	state::{ModuleSync, State},
	Module,
};

pub struct Camera {
	cfg: config::Camera,
	frame_chan: (mpsc::UnboundedSender<Mat>, mpsc::UnboundedReceiver<Mat>),
}

impl Camera {
	pub async fn new(cfg: config::Camera) -> Result<Self> {
		tokio::time::sleep(Duration::from_secs(3)).await;
		Ok(Camera {
			cfg,
			frame_chan: mpsc::unbounded_channel::<Mat>(),
		})
	}
}

#[async_trait]
impl Module for Camera {
	fn name(&self) -> &'static str {
		"camera"
	}

	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		let mat = self
			.frame_chan
			.1
			.recv()
			.await
			.ok_or(anyhow::anyhow!("no frame"))?;
		// let ball = find_best_contour(
		// 	&mat,
		// 	[0.0, 0.0, 0.0],
		// 	[0.0, 0.0, 0.0],
		// 	ball_heuristic(0.5, 0.5),
		// )?;
		// let yellow_goal = find_best_contour(
		// 	&mat,
		// 	[0.0, 0.0, 0.0],
		// 	[0.0, 0.0, 0.0],
		// 	ball_heuristic(1.0, 0.5),
		// )?;
		// let blue_goal = find_best_contour(
		// 	&mat,
		// 	[0.0, 0.0, 0.0],
		// 	[0.0, 0.0, 0.0],
		// 	ball_heuristic(1.0, 0.5),
		// )?;
		// sync.camera_notify.notify_waiters();
		{
			let mut frame = sync.frame.lock();
			frame.0 = mat;
			frame.1 = false;
		}
		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		let cfg = self.cfg.clone();
		let sender = self.frame_chan.0.clone();

		// spin up a new thread for the camera processing
		// because if you do it in the main tick task it will block everything
		// and won't be good for the health of the overall system.
		std::thread::spawn(move || {
			unsafe {
				ffi::initialize_camera(
					cfg.width,
					cfg.height,
					cfg.framerate,
					cfg.sensor_mode,
					cfg.shutter_speed,
				);
			}
			loop {
				let pkt = unsafe { ffi::get_image_packet() };
				let imslice = unsafe { slice::from_raw_parts_mut(pkt.data, pkt.len) };
				if let Ok(mat) = unsafe {
					Mat::new_nd_with_data(
						&[cfg.width as i32, cfg.height as i32],
						CV_8UC3,
						imslice.as_mut_ptr() as *mut c_void,
						None,
					)
				} {
					let _ = sender.send(mat);
				}
			}
		});
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}
