use std::f32::consts::PI;

use anyhow::Result;
use async_trait::async_trait;
use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::{
	config,
	math::{
		angles::distance,
		vec2::{dot, Vec2},
	},
};

use super::{state::State, Module};

pub struct Line {
	pub sensor_count: usize,
	pub pickup_threshold: usize,
	pub pickup_sensor_count: usize,
	pub trigger_threshold: usize,
	pub previous_vec: Option<Vec2>,
	pub serial: Option<SerialStream>,
}

impl Default for Line {
	fn default() -> Self {
		Self {
			sensor_count: 46,
			trigger_threshold: 400,
			pickup_threshold: 24,
			pickup_sensor_count: 10,
			previous_vec: None,
			serial: None,
		}
	}
}

pub fn angle_for_sensor(i: usize, length: usize) -> f32 {
	let percent = i as f32 / length as f32;
	percent * std::f32::consts::PI * 2.0
}

pub fn vec_for_sensor(i: usize, length: usize) -> Vec2 {
	let angle = angle_for_sensor(i, length);
	Vec2::from_rad(angle as f64)
}

impl Line {
	pub fn new(
		config::Line {
			baud_rate,
			pickup_sensor_count,
			pickup_threshold,
			sensor_count,
			trigger_threshold,
			uart_path,
		}: config::Line,
	) -> Result<Self> {
		let serial = tokio_serial::new(uart_path, baud_rate).open_native_async()?;

		Ok(Line {
			pickup_threshold,
			pickup_sensor_count,
			trigger_threshold,
			sensor_count,
			previous_vec: None,
			serial: Some(serial),
		})
	}
}

#[async_trait]
impl Module for Line {
	async fn tick(&mut self, state: &mut State) -> Result<()> {
		if let Some(ref mut serial) = self.serial {
			while serial.read_u8().await? != 255 {}
			let mut raw_data: Vec<u8> = vec![0; self.sensor_count];
			serial.read_exact(&mut raw_data).await?;
			raw_data.reverse();
			state.data.sensor_data = raw_data;
			state.line_detections = state
				.data
				.sensor_data
				.iter()
				.map(|&x| x > self.trigger_threshold as u8)
				.collect();
		}

		state.picked_up = did_pick_up(
			&state.data.sensor_data,
			self.pickup_threshold,
			self.pickup_sensor_count,
		);

		if state.picked_up {
			state.line_flipped = false;
		}

		let line_detections = state.line_detections.as_slice();
		let length = line_detections.len();
		let (a, b) = get_farthest_detections(line_detections);
		let (vec_a, vec_b) = (vec_for_sensor(a, length), vec_for_sensor(b, length));
		let mut vec = (vec_a + vec_b).normalize(); // add the vectors of both sensors.
		if vec.y == 0.0 && vec.x == 0.0 {
			// if the vector is zero, then the added vectors are perfectly perpendicular.
			let vec_a = vec_a + Vec2 { x: 1e-5, y: 0.0 };
			vec = (vec_a + vec_b).normalize();
		}

		if let Some(previous_vec) = self.previous_vec {
			if did_cross_line(vec, previous_vec) {
				state.line_flipped = !state.line_flipped;
			}
		}
		let koig_vec = if state.line_flipped { vec * -1.0 } else { vec };

		state.line_vector = koig_vec;

		// TODO: If line should run, then make the robot move away from the line.
		if let (true, _) = should_run(line_detections, state.line_flipped) {
			state.line_vector = koig_vec;
		}

		self.previous_vec = Some(vec);
		Ok(())
	}

	fn name(&self) -> &'static str {
		"line"
	}

	async fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}

// did_cross_line determines if two vectors indicate a line crossing.
pub fn did_cross_line(current_vec: Vec2, previous_vec: Vec2) -> bool {
	dot(current_vec, previous_vec) < 0.0
}

// did_pick_up determines if the robot has been picked up.
pub fn did_pick_up(
	line_values: &[u8],
	pickup_threshold: usize,
	pickup_sensor_count: usize,
) -> bool {
	line_values
		.iter()
		.map(|&x| x < pickup_threshold as u8)
		.filter(|&x| x)
		.count() >= pickup_sensor_count
}

/// should_run determines if koig ring (line avoidance) should run.
/// The following criteria are checked to determine if the line should be run:
/// 1. If two or more line sensors trigger, then the line avoidance should activate,
/// because it is the minimum amount of sensors needed to trigger in order to orient against the line.
///
/// 2. If the robot is currently outside of the line, then the line should ALWAYS be running
/// so that it can come back in.
pub fn should_run(triggers: &[bool], pointing_out: bool) -> (bool, usize) {
	let detections = triggers.iter().filter(|&x| *x).count();

	let should_run = detections >= 2 || (detections <= 1 && pointing_out);

	(should_run, detections)
}

/// get_farthest_detections returns the two indexes in the line sensor array that are the
/// most perpendicular from each other.
///
/// Specifically, this means that in the following ring configuration:
/// +-1-2--+
/// 0			 3
/// |			 |
/// +------+
///
/// It should return (0, 3), because it forms a perfect angle of 180 degrees, which is the most perpendicular.  
pub fn get_farthest_detections(detections: &[bool]) -> (usize, usize) {
	let mut first_detection = 0;
	let mut second_detection = 0;
	let mut closest_angle = 2.0 * PI;

	let triggered_only = detections
		.iter()
		.enumerate()
		.filter(|(_, &x)| x)
		.map(|(i, _)| i); // iterator of only triggered sensors

	for i in triggered_only.clone() {
		for j in triggered_only.clone() {
			let angle_1 = angle_for_sensor(i, detections.len());
			let angle_2 = angle_for_sensor(j, detections.len());
			let diff = distance(angle_1, angle_2);
			if PI - diff < closest_angle {
				closest_angle = PI - diff;
				first_detection = i;
				second_detection = j;
			}
		}
	}
	(first_detection, second_detection)
}
