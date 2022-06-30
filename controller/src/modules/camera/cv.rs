use std::f64::consts::PI;

use anyhow::Result;
use num::pow;
use opencv::{
	core::{in_range, no_array, Moments, Point, Point2d, Scalar, Vector},
	imgproc::{
		self, arc_length, contour_area, find_contours, is_contour_convex, CHAIN_APPROX_SIMPLE,
		RETR_TREE,
	},
	prelude::Mat,
	types::{VectorOfPoint, VectorOfVectorOfPoint},
};

pub type ColorBound = [f64; 3];
pub type ColorRange = (ColorBound, ColorBound);

#[must_use]
pub fn get_blob_centroid(moments: Moments) -> Point2d {
	Point2d::new(moments.m10 / moments.m00, moments.m01 / moments.m00)
}

pub fn mask(img: &Mat, lower_bound: ColorBound, upper_bound: ColorBound) -> Result<Mat> {
	let mut mask = Mat::default();
	in_range(
		&img,
		&Mat::from_slice(&lower_bound)?,
		&Mat::from_slice(&upper_bound)?,
		&mut mask,
	)?;
	Ok(mask)
}

pub fn find_best_contour<F>(
	img: &mut Mat,
	lower: ColorBound,
	upper: ColorBound,
	heuristic_fn: F,
) -> Result<Option<VectorOfPoint>>
where
	F: Fn(&VectorOfPoint) -> Result<f64>,
{
	let masked = mask(img, lower, upper)?;
	let mut contours = VectorOfVectorOfPoint::default();
	find_contours(
		&masked,
		&mut contours,
		RETR_TREE,
		CHAIN_APPROX_SIMPLE,
		Point::default(),
	)?;

	let mut contours: Vec<_> = contours
		.iter()
		.filter(|c| is_contour_convex(c).unwrap_or(false))
		.collect();

	contours.sort_by(|a, b| {
		heuristic_fn(a)
			.unwrap_or(0.0)
			.partial_cmp(&heuristic_fn(b).unwrap_or(0.0))
			.unwrap()
	});

	imgproc::draw_contours(
		img,
		&VectorOfVectorOfPoint::from(contours.clone()),
		-1,
		Scalar::new(255.0, 0.0, 0.0, 0.0),
		1,
		-1,
		&no_array(),
		0,
		Point::default(),
	)?;

	Ok(contours.first().cloned())
}

pub trait Heuristic = Fn(&VectorOfPoint) -> Result<f64>;

pub fn ball_heuristic(area_influence: f64, circularity_influence: f64) -> impl Heuristic {
	move |contour: &VectorOfPoint| {
		let area = contour_area(&contour, false)?;
		let perimeter = arc_length(&contour, true)?;
		let circularity = 4.0 * PI * area / pow(perimeter, 2);
		Ok((area * area_influence) * (circularity * circularity_influence))
	}
}
