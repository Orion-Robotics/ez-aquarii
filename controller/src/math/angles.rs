use std::f32::consts::PI;

pub fn distance(first: f32, second: f32) -> f32 {
	let phi = (first - second).abs() % (2.0 * PI);
	return if phi > PI { (2.0 * PI) - phi } else { phi };
}

#[test]
pub fn test_distance() {
	assert_eq!(distance(PI, 0.0), PI);
	assert_eq!(distance(PI + 0.1, 2.0 * PI), 2.0 * PI - (PI + 0.1));
}
