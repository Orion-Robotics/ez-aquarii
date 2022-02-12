use std::ops::{Add, Mul};

use serde::Serialize;

#[derive(Default, Clone, Copy, Serialize, PartialEq, Debug)]
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

pub fn dot(first: Vec2, second: Vec2) -> f64 {
	let angle = angle_between(first, second);
	first.magnitude() * second.magnitude() * angle.cos()
}

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Self {
		Vec2 { x, y }
	}

	pub fn from_rad(rad: f64) -> Self {
		Vec2 {
			x: rad.cos(),
			y: rad.sin(),
		}
	}

	pub fn angle_rad(&self) -> f64 {
		self.y.atan2(self.x)
	}

	pub fn magnitude(&self) -> f64 {
		self.x.hypot(self.y)
	}

	pub fn abs(&self) -> Self {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
		}
	}
}

impl Mul<f64> for Vec2 {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		return Vec2 {
			x: self.x * rhs,
			y: self.y * rhs,
		};
	}
}

impl Mul for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: Self) -> Self::Output {
		Vec2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl Add for Vec2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}
