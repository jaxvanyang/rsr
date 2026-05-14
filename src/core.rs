pub mod color;

pub use color::*;
use std::ops::*;

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(not(feature = "use_f64"))]
pub type Float = f32;

pub trait Number:
	Copy
	+ Neg<Output = Self>
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<Output = Self>
	+ Div<Output = Self>
	+ AddAssign
	+ SubAssign
	+ MulAssign
	+ DivAssign
	+ PartialOrd
	+ Default
{
	fn is_nan(self) -> bool {
		false
	}

	/// Check if two numbers are approximately equal with machine epsilon.
	fn approx_eq(self, rhs: Self) -> bool {
		self == rhs
	}

	fn abs(self) -> Self {
		if self < Self::default() { -self } else { self }
	}

	fn as_float(self) -> Float;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn min_num() -> Self;
	fn max_num() -> Self;
}

impl Number for i32 {
	fn as_float(self) -> Float {
		self as Float
	}

	fn min(self, rhs: Self) -> Self {
		Ord::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		Ord::max(self, rhs)
	}

	fn min_num() -> Self {
		Self::MIN
	}

	fn max_num() -> Self {
		Self::MAX
	}
}

impl Number for f32 {
	fn is_nan(self) -> bool {
		self.is_nan()
	}

	fn approx_eq(self, rhs: Self) -> bool {
		(self - rhs).abs() < Self::EPSILON
	}

	fn as_float(self) -> Float {
		self as Float
	}

	fn min(self, rhs: Self) -> Self {
		f32::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		f32::max(self, rhs)
	}

	fn min_num() -> Self {
		Self::MIN
	}

	fn max_num() -> Self {
		Self::MAX
	}
}

impl Number for f64 {
	fn is_nan(self) -> bool {
		self.is_nan()
	}

	fn approx_eq(self, rhs: Self) -> bool {
		(self - rhs).abs() < Self::EPSILON
	}

	fn as_float(self) -> Float {
		self as Float
	}

	fn min(self, rhs: Self) -> Self {
		f64::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		f64::max(self, rhs)
	}

	fn min_num() -> Self {
		Self::MIN
	}

	fn max_num() -> Self {
		Self::MAX
	}
}

pub fn lerp(t: Float, a: Float, b: Float) -> Float {
	debug_assert!((0.0..1.0).contains(&t));
	a + t * (b - a)
}
