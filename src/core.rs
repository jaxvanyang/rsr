pub mod color;

use approx::AbsDiffEq;
pub use color::*;
use std::ops::*;

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(not(feature = "use_f64"))]
pub type Float = f32;

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

pub fn next_float_up(mut v: Float) -> Float {
	if v.is_infinite() && v > 0.0 {
		return v;
	}
	if v == -0.0 {
		v = 0.0;
	}

	let mut bits = v.to_bits();
	if v >= 0.0 {
		bits += 1;
	} else {
		bits -= 1;
	}

	Float::from_bits(bits)
}

pub fn next_float_down(mut v: Float) -> Float {
	if v.is_infinite() && v < 0.0 {
		return v;
	}
	if v == 0.0 {
		v = -0.0;
	}

	let mut bits = v.to_bits();
	if v > 0.0 {
		bits -= 1;
	} else {
		bits += 1;
	}

	Float::from_bits(bits)
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Interval {
	low: Float,
	high: Float,
}

// TODO: floor(), celi(), quadratic()
impl Interval {
	/// # Examples
	///
	/// ```
	/// # use rsr::core::Interval;
	/// assert_eq!(Interval::new(1.0, 2.0), Interval::new(1.0, 2.0));
	/// assert_eq!(Interval::new(2.0, 1.0), Interval::new(1.0, 2.0));
	/// ```
	pub const fn new(low: Float, high: Float) -> Self {
		Self {
			low: low.min(high),
			high: high.max(low),
		}
	}

	pub fn new_with_error(value: Float, error: Float) -> Self {
		debug_assert!(error >= 0.0);

		if error == 0.0 {
			Self {
				low: value,
				high: value,
			}
		} else {
			Self {
				low: next_float_down(value - error),
				high: next_float_up(value + error),
			}
		}
	}

	pub fn in_range(self, other: Interval) -> bool {
		other.low <= self.low && self.high <= other.high
	}

	pub fn contains(self, value: Float) -> bool {
		(self.low..=self.high).contains(&value)
	}

	pub fn sqrt(self) -> Interval {
		Interval {
			low: next_float_down(self.low.sqrt()),
			high: next_float_up(self.high.sqrt()),
		}
	}

	pub fn fma(self, b: Interval, c: Interval) -> Interval {
		let lows = [
			self.low.mul_add(b.low, c.low),
			self.low.mul_add(b.high, c.low),
			self.high.mul_add(b.low, c.low),
			self.high.mul_add(b.high, c.low),
		];
		let highs = [
			self.low.mul_add(b.low, c.high),
			self.low.mul_add(b.high, c.high),
			self.high.mul_add(b.low, c.high),
			self.high.mul_add(b.high, c.high),
		];
		let low = lows.into_iter().reduce(|a, b| a.min(b)).unwrap();
		let high = highs.into_iter().reduce(|a, b| a.max(b)).unwrap();

		Interval {
			low: next_float_down(low),
			high: next_float_up(high),
		}
	}

	pub fn square(self) -> Interval {
		let mut low = self.low.abs();
		let mut high = self.high.abs();
		if low > high {
			(low, high) = (high, low);
		}

		if self.contains(0.0) {
			Self::new(0.0, next_float_up(high * high))
		} else {
			Self::new(next_float_down(low * low), next_float_up(high * high))
		}
	}

	pub fn lower_bound(&self) -> Float {
		self.low
	}

	pub fn upper_bound(&self) -> Float {
		self.high
	}

	pub fn midpoint(self) -> Float {
		(self.low + self.high) / 2.0
	}

	pub fn width(self) -> Float {
		self.high - self.low
	}
}

impl From<Float> for Interval {
	fn from(v: Float) -> Self {
		Self { low: v, high: v }
	}
}

impl From<Interval> for Float {
	fn from(v: Interval) -> Self {
		v.midpoint()
	}
}

impl PartialOrd for Interval {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.midpoint().partial_cmp(&other.midpoint())
	}
}

impl Neg for Interval {
	type Output = Interval;

	fn neg(self) -> Interval {
		Interval {
			low: -self.high,
			high: -self.low,
		}
	}
}

impl Add<Interval> for Interval {
	type Output = Interval;

	fn add(self, rhs: Interval) -> Interval {
		Interval {
			low: next_float_down(self.low + rhs.low),
			high: next_float_up(self.high + rhs.high),
		}
	}
}

impl AddAssign<Interval> for Interval {
	fn add_assign(&mut self, rhs: Interval) {
		*self = *self + rhs;
	}
}

impl Sub<Interval> for Interval {
	type Output = Interval;

	fn sub(self, rhs: Interval) -> Interval {
		Interval {
			low: next_float_down(self.low - rhs.high),
			high: next_float_up(self.high - rhs.low),
		}
	}
}

impl SubAssign<Interval> for Interval {
	fn sub_assign(&mut self, rhs: Interval) {
		*self = *self - rhs;
	}
}

impl Mul<Interval> for Interval {
	type Output = Interval;

	fn mul(self, rhs: Interval) -> Interval {
		let results = [
			self.low * rhs.low,
			self.high * rhs.low,
			self.low * rhs.high,
			self.high * rhs.high,
		];
		let low = results.into_iter().reduce(|a, b| a.min(b)).unwrap();
		let high = results.into_iter().reduce(|a, b| a.max(b)).unwrap();

		Interval {
			low: next_float_down(low),
			high: next_float_up(high),
		}
	}
}

impl MulAssign<Interval> for Interval {
	fn mul_assign(&mut self, rhs: Interval) {
		*self = *self * rhs;
	}
}

impl Div<Interval> for Interval {
	type Output = Interval;

	fn div(self, rhs: Interval) -> Interval {
		if rhs.low <= 0.0 && 0.0 <= rhs.high {
			return Interval {
				low: Float::NEG_INFINITY,
				high: Float::INFINITY,
			};
		}

		let results = [
			self.low / rhs.low,
			self.high / rhs.low,
			self.low / rhs.high,
			self.high / rhs.high,
		];
		let low = results.into_iter().reduce(|a, b| a.min(b)).unwrap();
		let high = results.into_iter().reduce(|a, b| a.max(b)).unwrap();

		Interval {
			low: next_float_down(low),
			high: next_float_up(high),
		}
	}
}

impl DivAssign<Interval> for Interval {
	fn div_assign(&mut self, rhs: Interval) {
		*self = *self / rhs;
	}
}

impl AbsDiffEq for Interval {
	type Epsilon = Float;

	fn default_epsilon() -> Self::Epsilon {
		Float::EPSILON
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		self.low.abs_diff_eq(&other.low, epsilon) && self.high.abs_diff_eq(&other.high, epsilon)
	}
}

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

impl Number for Interval {
	const ONE: Self = Self::new(1.0, 1.0);
	const MIN: Self = Self::new(Float::MIN, Float::MIN);
	const MAX: Self = Self::new(Float::MAX, Float::MAX);

	fn is_nan(self) -> bool {
		self.low.is_nan() || self.high.is_nan()
	}

	/// # Examples
	///
	/// ```
	/// # use rsr::core::{Interval, Number};
	/// assert_eq!(Interval::new(1.0, 2.0).abs(), Interval::new(1.0, 2.0));
	/// assert_eq!(Interval::new(-1.0, 2.0).abs(), Interval::new(0.0, 2.0));
	/// assert_eq!(Interval::new(-2.0, -1.0).abs(), Interval::new(1.0, 2.0));
	/// ```
	fn abs(self) -> Self {
		if self.low >= 0.0 {
			self
		} else if self.high <= 0.0 {
			Self::new(-self.high, -self.low)
		} else {
			Self::new(0.0, self.high.max(-self.low))
		}
	}

	fn as_float(self) -> Float {
		self.into()
	}

	fn min(self, rhs: Self) -> Self {
		if self.midpoint() < rhs.midpoint() {
			self
		} else {
			rhs
		}
	}

	fn max(self, rhs: Self) -> Self {
		if self.midpoint() < rhs.midpoint() {
			rhs
		} else {
			self
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;

	#[test]
	fn test_next_float() {
		assert!(next_float_up(0.0) > 0.0);
		assert!(next_float_down(0.0) < 0.0);
		assert!(next_float_up(1.0) > 1.0);
		assert!(next_float_down(1.0) < 1.0);
		assert!(next_float_up(-1.0) > -1.0);
		assert!(next_float_down(-1.0) < -1.0);
	}

	#[test]
	fn test_interval() {
		let a = Interval::new(-1.0, 2.0);
		let b = Interval::new(3.0, 4.0);
		let c = Interval::new(-5.0, 6.0);
		assert_abs_diff_eq!(a + b, Interval::new(2.0, 6.0), epsilon = 1e-6);
		assert_abs_diff_eq!(a - b, Interval::new(-5.0, -1.0), epsilon = 1e-6);
		assert_abs_diff_eq!(a * b, Interval::new(-4.0, 8.0), epsilon = 1e-6);
		assert_abs_diff_eq!(a / b, Interval::new(-1.0 / 3.0, 2.0 / 3.0), epsilon = 1e-6);
		assert_abs_diff_eq!(a.fma(b, c), Interval::new(-9.0, 14.0), epsilon = 1e-6);
		assert_abs_diff_eq!(a.square(), Interval::new(0.0, 4.0), epsilon = 1e-6);
		assert_abs_diff_eq!(b.square(), Interval::new(9.0, 16.0), epsilon = 1e-5);
		assert!(Interval::new(2.0, 6.0).in_range(a + b));
		assert!(Interval::new(-5.0, -1.0).in_range(a - b));
		assert!(Interval::new(4.0, 8.0).in_range(a * b));
		assert!(Interval::new(-1.0 / 3.0, 2.0 / 3.0).in_range(a / b));
		assert!(Interval::new(-9.0, 14.0).in_range(a.fma(b, c)));
		assert!(Interval::new(0.0, 4.0).in_range(a.square()));
		assert!(Interval::new(9.0, 16.0).in_range(b.square()));
	}
}
