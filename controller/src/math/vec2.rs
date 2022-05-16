use std::{
	fmt::Display,
	ops::{Add, Mul},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}

pub fn angle_between(first: Vec2, second: Vec2) -> f64 {
	let first_rad = first.angle_rad();
	let second_rad = second.angle_rad();
	second_rad - first_rad
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
		f64::atan2(self.y, self.x)
	}

	pub fn magnitude(&self) -> f64 {
		f64::hypot(self.x, self.y)
	}

	pub fn abs(&self) -> Self {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
		}
	}

	pub fn normalize(&self) -> Self {
		let magnitude = self.magnitude();
		Self {
			x: self.x / magnitude,
			y: self.y / magnitude,
		}
	}
}

impl Display for Vec2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("({}, {})", self.x, self.y))
	}
}

impl Mul<f64> for Vec2 {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec2 {
			x: self.x * rhs,
			y: self.y * rhs,
		}
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

#[test]
pub fn test_normalize() {
	let vec = Vec2::new(0.0, 5.0);
	assert_eq!(vec.normalize(), Vec2::new(0.0, 1.0));
}
