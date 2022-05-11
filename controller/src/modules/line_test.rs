use std::sync::Arc;

use crate::{
	math::vec2::Vec2,
	modules::{
		line::{self, Line},
		state::State,
		Module,
	},
};
use parking_lot::Mutex;
use test_case::test_case;

#[test_case(
  &[true, false, false, false, false, false, true, false, false, false],
  &[true, false, false, false, true, false, false, false, false, false],
  true;
  "simple move shouldn't flip"
)]
#[test_case(
  &[true, false, false, false, false, false, true, false, false, false],
  &[true, false, false, false, true, false, false, false, false, false],
  true;
  "shouldn't koig here"
)]
#[test_case(
  &[false,true,false,false,false,true,true,false,false,true],
  &[false,true,true,false,true,true,false,false,false,false],
  true;
  "moving further into corner"
)]
#[test_case(
  &[true,false,false,false,false,false,true,true,false,true],
  &[false,true,false,false,false,true,true,false,false,true],
  false;
  "moving into corner"
)]
#[tokio::test]
pub async fn test_flips(first: &[bool], second: &[bool], flip: bool) {
	let mutex = Arc::new(Mutex::new(State::default()));
	let mut line = Line::default();
	let mut state = mutex.lock();
	state.line_detections = Vec::from(first);
	line.tick(&mut Arc::clone(&mutex)).await.unwrap();
	state.print_state();
	state.line_detections = Vec::from(second);
	line.tick(&mut Arc::clone(&mutex)).await.unwrap();
	state.print_state();

	assert_eq!(state.line_flipped, flip);
}

#[test_case(Vec2::new(-0.1, 0.0), Vec2::new(0.1, 0.0), true; "crosses line when crosses axis")]
#[test_case(Vec2::new(0.2, 0.0), Vec2::new(0.1, 0.0), false; "does not cross when on same side")]
pub fn test_line_cross_over(a: Vec2, b: Vec2, expected: bool) {
	assert_eq!(line::did_cross_line(a, b), expected);
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
	let (should_run, detection_count) = line::should_run(triggers, pointing_out);
	assert_eq!(should_run, expected);
	assert_eq!(detection_count, expected_detections);
}

#[test_case(&[true, false, false, false, false, true], (0, 5); "6 sensors, 2 activated")]
#[test_case(&[true, false, false, true, false, false, true], (0, 3); "7 sensors, 3 activated")]
#[test_case(&[true, true, true, true, true, true, true], (0, 3); "7 sensors, 7 activated")]
pub fn test_line_get_farthest_detections(sensors: &[bool], expected: (usize, usize)) {
	assert_eq!(line::get_farthest_detections(sensors), Some(expected));
}

#[test_case(&[5, 5, 10, 25], 15, 3, true; "3 under threshold, 3 required")]
#[test_case(&[5, 5, 10, 25], 15, 4, false; "3 under threshold, 4 required")]
#[test_case(&[25, 25, 25, 25], 15, 4, false; "0 under threshold, 4 required")]
pub fn test_line_did_pick_up(
	line_values: &[u8],
	pickup_threshold: usize,
	pickup_sensor_count: usize,
	expected: bool,
) {
	assert_eq!(
		line::did_pick_up(line_values, pickup_threshold, pickup_sensor_count),
		expected
	);
}

// #[tokio::test]
// pub async fn test_weird_line() {
// 	let mut state = State::default();
// 	let mut line = Line::default();

// 	let initial = vec![
// 		false, true, false, false, false, true, true, false, false, true,
// 	];

// 	state.line_detections = initial.clone();
// 	line.tick(&mut state).await.unwrap();

// 	let next = vec![
// 		false, true, true, false, true, true, false, false, false, false,
// 	];
// 	state.line_detections = next.clone();
// 	line.tick(&mut state).await.unwrap();

// 	state.print_state();
// }
