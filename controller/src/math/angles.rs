use std::f32::consts::PI;

/// here be dragons
pub fn true_angle(angle: f64) -> f64 {
	((5.0 * std::f64::consts::PI / 2.0) - angle) % (2.0 * std::f64::consts::PI)
}

pub fn make_bipolar(angle: f64) -> f64 {
	if angle > std::f64::consts::PI {
		angle - 2.0 * std::f64::consts::PI
	} else {
		angle
	}
}

pub fn distance(first: f32, second: f32) -> f32 {
	let phi = (first - second).abs() % (2.0 * PI);
	if phi > PI {
		(2.0 * PI) - phi
	} else {
		phi
	}
}

#[test]
pub fn test_distance() {
	assert_eq!(distance(PI, 0.0), PI);
	assert_eq!(distance(PI + 0.1, 2.0 * PI), 2.0 * PI - (PI + 0.1));
}
