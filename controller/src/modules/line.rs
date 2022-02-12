use std::f32::consts::PI;

use crate::math::{
	angles::distance,
	vec2::{dot, Vec2},
};
use anyhow::Result;
use async_trait::async_trait;
use cached::proc_macro::cached;
use test_case::test_case;

use super::{state::State, Module};

pub struct Line {
	pub sensor_count: usize,
	pub pickup_threshold: usize,
	pub previous_vec: Option<Vec2>,
}

#[cached]
pub fn angle_for_sensor(i: usize, length: usize) -> f32 {
	let percent = i as f32 / length as f32;
	percent * std::f32::consts::PI * 2.0
}

#[cached]
pub fn vec_for_sensor(i: usize, length: usize) -> Vec2 {
	let angle = angle_for_sensor(i, length);
	Vec2::from_rad(angle as f64)
}

impl Default for Line {
	fn default() -> Self {
		Line {
			sensor_count: 42,
			pickup_threshold: 24,
			previous_vec: None,
		}
	}
}

impl Line {
	pub fn new(pickup_threshold: usize, sensor_count: usize) -> Self {
		Line {
			pickup_threshold,
			sensor_count,
			previous_vec: None,
		}
	}

	// did_cross_line determines if two vectors indicate a line crossing.
	pub fn did_cross_line(&self, current_vec: Vec2, previous_vec: Vec2) -> bool {
		dot(current_vec, previous_vec) < 0.0
	}

	/// should_run determines if koig ring (line avoidance) should run.
	/// The following criteria are checked to determine if the line should be run:
	/// 1. If two or more line sensors trigger, then the line avoidance should activate,
	/// because it is the minimum amount of sensors needed to trigger in order to orient against the line.
	///
	/// 2. If the robot is currently outside of the line, then the line should ALWAYS be running
	/// so that it can come back in.
	pub fn should_run(&self, triggers: &[bool], pointing_out: bool) -> (bool, usize) {
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
	pub fn get_farthest_detections(&self, detections: &[bool]) -> (usize, usize) {
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
}

#[async_trait]
impl Module for Line {
	async fn tick(&mut self, state: &mut State) -> Result<()> {
		let line_detections = state.line_detections.as_slice();
		let length = line_detections.len();
		let (a, b) = self.get_farthest_detections(line_detections);
		let (vec_a, vec_b) = (vec_for_sensor(a, length), vec_for_sensor(b, length));
		let vec = vec_a + vec_b; // add the vectors of both sensors.

		if let Some(previous_vec) = self.previous_vec {
			if self.did_cross_line(vec, previous_vec) {
				state.line_flipped = !state.line_flipped;
			}
		}

		let vec = if state.line_flipped { vec * -1.0 } else { vec };

		state.line_vector = vec;
		// TODO: If line should run, then make the robot move away from the line.
		if let (true, _) = self.should_run(line_detections, state.line_flipped) {
			state.move_vector = vec;
		}

		self.previous_vec = Some(vec);
		Ok(())
	}

	fn name(&self) -> &'static str {
		"line"
	}

	fn start(&mut self) -> Result<()> {
		Ok(())
	}

	async fn stop(&mut self) -> Result<()> {
		Ok(())
	}
}

#[tokio::test]
pub async fn test_tick() {
	let mut state = State::default();
	let mut line = Line::default();

	state.line_detections = vec![
		true, false, false, false, false, true, false, false, true, false,
	];

	line.tick(&mut state).await.unwrap();
	let old_vec = state.line_vector.clone();

	state.line_detections = vec![
		true, false, false, false, true, false, false, false, true, false,
	];
	line.tick(&mut state).await.unwrap();
	let new_vec = state.line_vector.clone();

	assert_eq!(old_vec, new_vec);

	state.line_detections = vec![
		true, false, false, false, false, true, false, false, true, false,
	];
	line.tick(&mut state).await.unwrap();

	assert_ne!(old_vec, new_vec);
}

#[test_case(Vec2::new(-0.1, 0.0), Vec2::new(0.1, 0.0), true; "crosses line when crosses axis")]
#[test_case(Vec2::new(0.2, 0.0), Vec2::new(0.1, 0.0), false; "does not cross when on same side")]
pub fn test_line_cross_over(a: Vec2, b: Vec2, expected: bool) {
	let line = Line::default();
	assert_eq!(line.did_cross_line(a, b), expected);
}

#[test_case(&[true, false, true, false, false], true, true, 2; "2 sensors, pointing out")]
#[test_case(&[true, false, true, false, false], true, true, 2; "2 sensors, pointing in")]
#[test_case(&[true, false, false, false, false], true, true, 1; "1 sensor, pointing out")]
#[test_case(&[true, false, false, false, false], false, false, 1; "1 sensor, pointing in")]
pub fn test_line_should_run(
	triggers: &[bool],
	pointing_out: bool,
	expected: bool,
	expected_detections: usize,
) {
	let line = Line::default();
	let (should_run, detection_count) = line.should_run(triggers, pointing_out);
	assert_eq!(should_run, expected);
	assert_eq!(detection_count, expected_detections);
}

#[test_case(&[true, false, false, false, false, true], (0, 5); "6 sensors, 2 activated")]
#[test_case(&[true, false, false, true, false, false, true], (0, 3); "7 sensors, 3 activated")]
#[test_case(&[true, true, true, true, true, true, true], (0, 3); "7 sensors, 7 activated")]
pub fn test_line_get_farthest_detections(sensors: &[bool], expected: (usize, usize)) {
	let line = Line::default();
	assert_eq!(line.get_farthest_detections(sensors), expected);
}
