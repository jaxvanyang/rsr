pub mod color;

pub use color::*;
use std::ops::*;

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(not(feature = "use_f64"))]
pub type Float = f32;

pub trait Number:
	Copy
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<Output = Self>
	+ Div<Output = Self>
	+ AddAssign
	+ SubAssign
	+ MulAssign
	+ DivAssign
	+ PartialOrd
{
	fn abs(self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;

	/// Check if two numbers are approximately equal with machine epsilon.
	fn approx_eq(self, rhs: Self) -> bool {
		self == rhs
	}
}

impl Number for i32 {
	fn abs(self) -> Self {
		i32::abs(self)
	}

	fn min(self, rhs: Self) -> Self {
		Ord::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		Ord::max(self, rhs)
	}
}

impl Number for f32 {
	fn abs(self) -> Self {
		f32::abs(self)
	}

	fn min(self, rhs: Self) -> Self {
		f32::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		f32::max(self, rhs)
	}

	fn approx_eq(self, rhs: Self) -> bool {
		(self - rhs).abs() < Self::EPSILON
	}
}

impl Number for f64 {
	fn abs(self) -> Self {
		f64::abs(self)
	}

	fn min(self, rhs: Self) -> Self {
		f64::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		f64::max(self, rhs)
	}

	fn approx_eq(self, rhs: Self) -> bool {
		(self - rhs).abs() < Self::EPSILON
	}
}
