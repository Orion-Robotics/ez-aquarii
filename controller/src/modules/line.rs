use std::{f32::consts::PI, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use parking_lot::{Mutex, RwLock};

use crate::{
	config,
	math::{
		angles::distance,
		vec2::{dot, Vec2},
	},
};

use super::{
	state::{ModuleSync, State},
	Module,
};

#[derive(Default)]
pub struct Line {}

pub fn angle_for_sensor(i: usize, length: usize) -> f32 {
	let percent = i as f32 / length as f32;
	percent * std::f32::consts::PI * 2.0
}

pub fn vec_for_sensor(i: usize, length: usize) -> Vec2 {
	let angle = angle_for_sensor(i, length);
	Vec2::from_rad(angle as f64)
}

impl Line {
	pub fn new(config::Line { .. }: config::Line) -> Result<Self> {
		Ok(Line {})
	}
}

#[async_trait]
impl Module for Line {
	async fn tick(&mut self, state: &mut Arc<RwLock<State>>, sync: &mut ModuleSync) -> Result<()> {
		sync.reader_notify.notified().await;
		let config::Line {
			pickup_threshold,
			pickup_sensor_count,
			trigger_threshold,
			..
		} = state.read().config.line.as_ref().unwrap().to_owned();

		let mut state = state.write();

		state.line_detections = state
			.data
			.sensor_data
			.iter()
			.map(|&x| x > trigger_threshold as u8)
			.collect();

		state.picked_up = did_pick_up(
			&state.data.sensor_data,
			pickup_threshold,
			pickup_sensor_count,
		);

		if state.picked_up {
			state.line_flipped = false;
		}

		let line_detections = state.line_detections.as_slice();
		let length = line_detections.len();

		// TODO: If line should run, then make the robot move away from the line.
		match should_run(line_detections, state.line_flipped) {
			(true, detections) => {
				// if there are no detections, then the current line vector should be set to the previous vector.
				// situation: out of field.
				if detections == 0 {
					state.line_vector = state.previous_vec.map(|x| x * -1.0);
				} else {
					let (a, b) = get_farthest_detections(line_detections).unwrap();
					let (vec_a, vec_b) = (vec_for_sensor(a, length), vec_for_sensor(b, length));
					let mut towards_line = (vec_a + vec_b).normalize(); // add the vectors of both sensors.

					// this is because if you look at the line PCB from the top down
					// then the x axis is flipped, so we have to flip y.
					// the line sensors start on the left (or right maybe who knows)
					// and go clockwise... so reverse trig plane...
					// so we need to flip just the y axis to fix it :)
					// NO LOGNER NEEDED FOR SOME REASON
					// towards_line = Vec2 {
					// 	x: towards_line.x,
					// 	y: -towards_line.y,
					// };
					// towards_line = Vec2 {
					// 	x: -towards_line.x,
					// 	y: towards_line.y,
					// };

					if towards_line.y == 0.0 && towards_line.x == 0.0 {
						// if the vector is zero, then the added vectors are perfectly perpendicular.
						let vec_a = vec_a + Vec2 { x: 1e-5, y: 0.0 };
						towards_line = (vec_a + vec_b).normalize();
					}

					if let Some(previous_vec) = state.previous_vec {
						if did_cross_line(towards_line, previous_vec) {
							state.line_flipped = !state.line_flipped;
							tracing::debug!(
								"crossed line {} {:?}",
								state.line_flipped,
								towards_line
							);
						}
					}
					let koiged_vec = if state.line_flipped {
						towards_line * -1.0
					} else {
						towards_line
					};

					state.line_vector = Some(koiged_vec);
					state.previous_vec = Some(towards_line);
				}
			}
			(false, _) => {
				tracing::debug!("line no longer flipped, since shouldn't run");
				state.line_flipped = false;
				state.previous_vec = None;
				state.line_vector = None;
			}
		}

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

/// did_cross_line determines if two vectors indicate a line crossing.
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
/// 0      3
/// |      |
/// +------+
///
/// It should return (0, 3), because it forms a perfect angle of 180 degrees, which is the most perpendicular.  
pub fn get_farthest_detections(detections: &[bool]) -> Option<(usize, usize)> {
	let mut first_detection: Option<usize> = None;
	let mut second_detection: Option<usize> = None;
	let mut closest_angle = 2.0 * PI;

	let triggered_only: Vec<_> = detections
		.iter()
		.enumerate()
		.filter(|(_, &x)| x)
		.map(|(i, _)| i)
		.collect(); // iterator of only triggered sensors

	if triggered_only.is_empty() {
		return None;
	}

	for &i in &triggered_only {
		for &j in &triggered_only {
			let angle_1 = angle_for_sensor(i, detections.len());
			let angle_2 = angle_for_sensor(j, detections.len());
			let diff = distance(angle_1, angle_2);
			if PI - diff < closest_angle {
				closest_angle = PI - diff;
				first_detection = Some(i);
				second_detection = Some(j);
			}
		}
	}
	Some((
		first_detection.or(second_detection).unwrap(),
		second_detection.or(first_detection).unwrap(),
	))
}
