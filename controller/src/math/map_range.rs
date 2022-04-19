use num::Num;

pub trait MapRange
where
	Self: Num,
{
	fn map_range(&self, from_range: (Self, Self), to_range: (Self, Self)) -> Self;
}

impl<I> MapRange for I
where
	I: Copy + Num,
{
	fn map_range(&self, from_range: (I, I), to_range: (I, I)) -> I {
		to_range.0
			+ (*self - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
	}
}
