pub mod cv;
use crate::{
	config::{self, CameraConfig, Thresholds},
	ffi,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use cv::{ball_heuristic, find_best_contour};
use opencv::{
	core::CV_8UC3,
	prelude::{Mat, MatTraitConstManual},
};
use parking_lot::RwLock;
use std::{ffi::c_void, slice, sync::Arc};
use tokio::sync::mpsc;
// use std::fs::OpenOptions;
// use std::io::Write;

use self::cv::{loc, ColorBound};

use super::{
	state::{Blob, ModuleSync, State},
	Module,
};

pub struct Camera {
	cfg: config::CameraConfig,
	frame_chan: (mpsc::UnboundedSender<Mat>, mpsc::UnboundedReceiver<Mat>),
}

impl Camera {
	pub async fn new(cfg: config::CameraConfig) -> Result<Self> {
		Ok(Camera {
			cfg,
			frame_chan: mpsc::unbounded_channel(),
		})
	}
}

#[async_trait]
impl Module for Camera {
	fn name(&self) -> &'static str {
		"camera"
	}

	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		let CameraConfig {
			bypass,
			saturation,
			thresholds,
			..
		} = state.read().config.camera.as_ref().unwrap().clone();
		let Thresholds { ball, yellow, blue } = thresholds;

		let mat = self
			.frame_chan
			.1
			.recv()
			.await
			.ok_or_else(|| anyhow::anyhow!("no frame"))?;

		let size = mat.size()?;
		let cx = (size.width / 2) as f64;
		let cy = (size.height / 2) as f64;

		let (ball, yellow, blue) = futures::try_join!(
			{
				let mut mat = mat.clone();
				tokio::task::spawn_blocking::<_, Result<(Mat, Option<Mat>)>>(move || {
					let result = find_best_contour(
						&mut mat,
						ball.lower.to_array(),
						ball.upper.to_array(),
						10.0,
						ball_heuristic(0.1, 1.0),
						(235.0, 131.0, 52.0),
					)?;
					Ok((mat, result))
				})
			},
			{
				let mut mat = mat.clone();
				tokio::task::spawn_blocking::<_, Result<(Mat, Option<Mat>)>>(move || {
					let result = find_best_contour(
						&mut mat,
						yellow.lower.to_array(),
						yellow.upper.to_array(),
						10.0,
						ball_heuristic(0.5, 0.5),
						(230.0, 225.0, 73.0),
					)?;
					Ok((mat, result))
				})
			},
			{
				let mut mat = mat.clone();
				tokio::task::spawn_blocking::<_, Result<(Mat, Option<Mat>)>>(move || {
					let result = find_best_contour(
						&mut mat,
						blue.lower.to_array(),
						blue.upper.to_array(),
						10.0,
						ball_heuristic(0.5, 0.5),
						(73.0, 128.0, 230.0),
					)?;
					Ok((mat, result))
				})
			},
		)?;
		let ((ball_mat, ball_contour), (yellow_mat, yellow_contour), (blue_mat, blue_contour)) = (
			ball.context("failed to get ball contour")?,
			yellow.context("failed to get yellow contour")?,
			blue.context("failed to get blue contour")?,
		);

		// convenience function that or's two mats into one
		let or = |src1: Mat, src2: Mat| -> Result<_> {
			let mut output = Mat::default();
			opencv::core::bitwise_or(&src1, &src2, &mut output, &Mat::default())
				.context("failed to bitwise or")?;
			Ok(output)
		};

		let masked = if bypass {
			mat
		} else {
			let masked = or(ball_mat, yellow_mat)?;
			or(masked, blue_mat)?
		};

		state.write().camera_data.ball = if let Some(ball_contour) = ball_contour {
			Some(loc(ball_contour, (cx, cy))?)
		} else {
			None
		};
		state.write().camera_data.yellow_goal = if let Some(yellow_contour) = yellow_contour {
			Some(loc(yellow_contour, (cx, cy))?)
		} else {
			None
		};
		state.write().camera_data.blue_goal = if let Some(blue_contour) = blue_contour {
			Some(loc(blue_contour, (cx, cy))?)
		} else {
			None
		};

		{
			let mut frame = sync.frame.lock();
			*frame = masked;
			sync.camera_notify.notify_waiters();
		}
		Ok(())
	}

	async fn start(&mut self) -> Result<()> {
		let cfg = self.cfg.clone();

		// spin up a new thread for the camera processing
		// because if you do it in the main tick task it will block everything
		// and won't be good for the health of the overall system.
		let sender = self.frame_chan.0.clone();
		std::thread::spawn(move || {
			unsafe {
				ffi::initialize_camera(
					cfg.width,
					cfg.height,
					cfg.framerate,
					cfg.sensor_mode,
					cfg.shutter_speed,
					cfg.balance_red,
					cfg.balance_blue,
					cfg.saturation,
					cfg.brightness,
					cfg.exposure,
					cfg.iso,
				);
			}
			loop {
				let pkt = unsafe { ffi::get_image_packet() };
				let imslice = unsafe { slice::from_raw_parts_mut(pkt.data, pkt.len) };
				if let Ok(mat) = unsafe {
					Mat::new_nd_with_data(
						&[cfg.width as i32, cfg.height as i32],
						CV_8UC3,
						imslice.as_mut_ptr().cast::<std::ffi::c_void>(),
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
