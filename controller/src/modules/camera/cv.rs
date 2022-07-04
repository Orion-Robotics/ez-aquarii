use std::f64::consts::PI;

use anyhow::{Context, Result};
use num::pow;
use opencv::{
	core::{in_range, no_array, Moments, Point, Point2d, VecN, BORDER_CONSTANT, CV_8UC1},
	imgproc::{
		self, arc_length, contour_area, find_contours, is_contour_convex, moments,
		CHAIN_APPROX_NONE, CHAIN_APPROX_SIMPLE, FILLED, LINE_4, RETR_TREE,
	},
	prelude::{Mat, MatTraitConstManual},
	types::{VectorOfMat, VectorOfPoint, VectorOfVectorOfPoint},
};

use crate::modules::state::Blob;

pub type ColorBound = [u8; 3];
pub type ColorRange = (ColorBound, ColorBound);

pub fn get_blob_centroid(moments: Moments) -> Point2d {
	Point2d::new(moments.m10 / moments.m00, moments.m01 / moments.m00)
}

pub fn loc(contour: Mat, center: (f64, f64)) -> Result<Blob> {
	let centroid = get_blob_centroid(moments(&contour, false)?);
	let dist_x = centroid.x - center.0;
	let dist_y = centroid.y - center.1;
	let angle = f64::atan2(dist_y, dist_x);
	let distance = f64::sqrt(dist_x.powi(2) + dist_y.powi(2));
	Ok(Blob { distance, angle })
}

pub fn mask(
	img: &Mat,
	lower_bound: ColorBound,
	upper_bound: ColorBound,
	erode: bool,
) -> Result<Mat> {
	let mut mask = Mat::default();
	in_range(
		&img,
		&Mat::from_slice(&lower_bound)?,
		&Mat::from_slice(&upper_bound)?,
		&mut mask,
	)?;
	let mut mask = if erode {
		let mut eroded = Mat::default();
		opencv::imgproc::dilate(
			&mask,
			&mut eroded,
			&Mat::ones(10, 10, CV_8UC1)?,
			Point::new(-1, -1),
			3,
			BORDER_CONSTANT,
			VecN::default(),
		)?;
		opencv::imgproc::erode(
			&mask,
			&mut eroded,
			&Mat::ones(10, 10, CV_8UC1)?,
			Point::new(-1, -1),
			3,
			BORDER_CONSTANT,
			VecN::default(),
		)?;
		eroded
	} else {
		mask
	};
	Ok(mask)
}

pub fn find_best_contour<F>(
	img: &mut Mat,
	lower: ColorBound,
	upper: ColorBound,
	min_size: f64,
	heuristic_fn: F,
	color: (f64, f64, f64),
) -> Result<Option<Mat>>
where
	F: Fn(&Mat) -> Result<f64>,
{
	let masked = mask(img, lower, upper, true).context("failed to mask image")?;

	let mut contours: Vec<_> = {
		let mut raw_contours = VectorOfMat::default();
		find_contours(
			&masked,
			&mut raw_contours,
			RETR_TREE,
			CHAIN_APPROX_NONE,
			Point::default(),
		)
		.context("failed to find contours")?;
		raw_contours
			.iter()
			.filter(|x| {
				let x = contour_area(x, false).unwrap_or(0.0);
				x > min_size
			})
			.collect()
	};

	contours.sort_by(|a, b| {
		let first = heuristic_fn(a).unwrap_or(0.0);
		let second = heuristic_fn(b).unwrap_or(0.0);
		let h = first.partial_cmp(&second);

		if h == None {
			tracing::debug!("ordering is none, {:?} {:?}", first, second);
		}

		h.unwrap()
	});

	let mut out = Mat::default();
	opencv::core::bitwise_and(img, img, &mut out, &masked)?;

	imgproc::draw_contours(
		&mut out,
		&VectorOfMat::from(contours.clone()),
		-1,
		opencv::core::Scalar::new(color.2, color.1, color.0, 0.0),
		2,
		FILLED,
		&no_array(),
		i32::MAX,
		Point::default(),
	)
	.context("failed to draw contours")?;

	let biggest_contour = contours.last().cloned();

	if let Some(contour) = biggest_contour {
		let size = img.size()?;
		let centroid = get_blob_centroid(moments(&contour, false)?);
		imgproc::line(
			&mut out,
			Point::new(size.width / 2, size.height / 2),
			Point::new(centroid.x as i32, centroid.y as i32),
			opencv::core::Scalar::new(color.2, color.1, color.0, 0.0),
			LINE_4,
			0,
			0,
		)?;
	}

	*img = out;
	// select only the biggest contour
	Ok(contours.first().cloned())
}

pub trait Heuristic = Fn(&Mat) -> Result<f64>;

pub fn ball_heuristic(area_influence: f64, circularity_influence: f64) -> impl Heuristic {
	move |contour: &Mat| {
		let area = contour_area(&contour, false)?;
		let perimeter = arc_length(&contour, true)?;
		let circularity = 4.0 * PI * area / pow(perimeter, 2);
		// tracing::debug!(
		// 	"ball heuristic values {} {} {}",
		// 	area,
		// 	perimeter,
		// 	circularity
		// );
		Ok((area * area_influence) * (circularity * circularity_influence))
	}
}
