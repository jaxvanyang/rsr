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
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;

	fn is_nan(self) -> bool {
		false
	}

	fn abs(self) -> Self {
		if self < Self::default() { -self } else { self }
	}

	fn as_float(self) -> Float;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
}

impl Number for i32 {
	const ONE: Self = 1;
	const MIN: Self = i32::MIN;
	const MAX: Self = i32::MAX;

	fn as_float(self) -> Float {
		self as Float
	}

	fn min(self, rhs: Self) -> Self {
		Ord::min(self, rhs)
	}

	fn max(self, rhs: Self) -> Self {
		Ord::max(self, rhs)
	}
}

impl Number for f32 {
	const ONE: Self = 1.0;
	const MIN: Self = f32::MIN;
	const MAX: Self = f32::MAX;

	fn is_nan(self) -> bool {
		self.is_nan()
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
}

impl Number for f64 {
	const ONE: Self = 1.0;
	const MIN: Self = f64::MIN;
	const MAX: Self = f64::MAX;

	fn is_nan(self) -> bool {
		self.is_nan()
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
}

pub fn lerp(t: Float, a: Float, b: Float) -> Float {
	debug_assert!((0.0..1.0).contains(&t));
	a + t * (b - a)
}

pub fn diff_of_products(a: Float, b: Float, c: Float, d: Float) -> Float {
	let cd = c * d;
	let result = a.mul_add(b, -cd);
	let error = (-c).mul_add(d, cd);
	result + error
}
