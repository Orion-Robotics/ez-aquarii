pub mod cv;
use crate::{
	config::{self, Thresholds},
	ffi,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use cv::{ball_heuristic, find_best_contour};
use opencv::{core::CV_8UC3, prelude::Mat, types::VectorOfPoint};
use parking_lot::RwLock;
use std::{ffi::c_void, slice, sync::Arc};
use tokio::sync::mpsc;
// use std::fs::OpenOptions;
// use std::io::Write;

use self::cv::ColorBound;

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
		let Thresholds { ball, yellow, blue } =
			state.read().config.camera.as_ref().unwrap().thresholds;

		let mat = self
			.frame_chan
			.1
			.recv()
			.await
			.ok_or_else(|| anyhow::anyhow!("no frame"))?;

		let (ball, yellow, blue) = futures::try_join!(
			{
				let mut mat = mat.clone();
				tokio::task::spawn_blocking::<_, Result<(Mat, Option<VectorOfPoint>)>>(move || {
					let result = find_best_contour(
						&mut mat,
						ball.lower.to_array(),
						ball.upper.to_array(),
						ball_heuristic(0.5, 0.5),
					)?;
					Ok((mat, result))
				})
			},
			{
				let mut mat = mat.clone();
				tokio::task::spawn_blocking::<_, Result<(Mat, Option<VectorOfPoint>)>>(move || {
					let result = find_best_contour(
						&mut mat,
						yellow.lower.to_array(),
						yellow.upper.to_array(),
						ball_heuristic(0.5, 0.5),
					)?;
					Ok((mat, result))
				})
			},
			{
				let mut mat = mat.clone();
				tokio::task::spawn_blocking::<_, Result<(Mat, Option<VectorOfPoint>)>>(move || {
					let result = find_best_contour(
						&mut mat,
						blue.lower.to_array(),
						blue.upper.to_array(),
						ball_heuristic(0.5, 0.5),
					)?;
					Ok((mat, result))
				})
			},
		)?;
		let ((ball_mat, ball_contours), (yellow_mat, yellow_contours), (blue_mat, blue_contours)) = (
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

		let masked = or(ball_mat, yellow_mat)?;
		let masked = or(masked, blue_mat)?;

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
