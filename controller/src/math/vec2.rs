use serde::Serialize;

#[derive(Default, Clone, Copy, Serialize)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}

pub fn angle_between(first: Vec2, second: Vec2) -> f64 {
	let first_rad = first.angle_rad();
	let second_rad = second.angle_rad();
	let diff = second_rad - first_rad;
	return diff;
}

impl Vec2 {
	fn angle_rad(&self) -> f64 {
		self.y.atan2(self.x)
	}
}
