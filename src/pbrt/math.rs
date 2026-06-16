use super::vecmath::Vector2f;
use crate::Float;

cfg_select! {
	feature = "use_f64" => {
		pub use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};
	}
	_ => {
		pub use std::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};
	}
}

/// # Examples
///
/// ```
/// # use rsr::polynomial;
/// assert_eq!(polynomial!(2.0, 1.0), 1.0);
/// // x = 2, f(x) = 1 + 2 * x + 3 * x^2
/// assert_eq!(polynomial!(2_f32, 1_f32, 2_f32), 5_f32);
/// assert_eq!(polynomial!(2_f32, 1_f32, 2_f32, 3_f32), 17_f32);
/// assert_eq!(polynomial!(2_f64, 1_f64, 2_f64, 3_f64), 17_f64);
/// ```
#[macro_export]
macro_rules! polynomial {
	($x:expr, $c:expr) => {
		$c
	};
	($x:expr, $c0:expr, $($c:expr),+) => {
		$x.mul_add(polynomial!($x, $($c),+), $c0)
	};
}

/// Return `a + t * (b - a)`.
///
/// # Examples
///
/// ```
/// # use rsr::pbrt::math::lerp;
/// assert_eq!(lerp(0.3, 0.0, 1.0), 0.3);
/// ```
pub fn lerp(t: Float, a: Float, b: Float) -> Float {
	debug_assert!((0.0..=1.0).contains(&t));
	a + t * (b - a)
}

/// # Examples
///
/// ```
/// # use rsr::pbrt::math::fast_exp;
/// # use approx::assert_abs_diff_eq;
/// assert_eq!(fast_exp(0.0), 1.0);
/// assert_abs_diff_eq!(fast_exp(2.0), 2_f32.exp(), epsilon = 1e-3);
/// assert_abs_diff_eq!(fast_exp(-2.0), (-2_f32).exp(), epsilon = 1e-3);
/// ```
#[allow(clippy::excessive_precision)]
pub fn fast_exp(x: f32) -> f32 {
	// https://www.pbr-book.org/4ed/Utilities/Mathematical_Infrastructure#LogarithmsandExponentiation
	let xp = x * std::f32::consts::LOG2_E;
	let fxp = xp.floor();
	let f = xp - fxp;
	let i = fxp as i32;
	let two_to_f = polynomial!(f, 1., 0.695556856, 0.226173572, 0.0781455737);
	let exp = get_exponent(two_to_f) + i;

	if exp < -126 {
		return 0.0;
	}
	if exp > 127 {
		return f32::INFINITY;
	}

	let mut bits = two_to_f.to_bits();
	bits &= 0b10000000011111111111111111111111;
	bits |= ((exp + 127) as u32) << 23;

	f32::from_bits(bits)
}

/// Return the exponent of IEEE 754 single-precision floating-point number `v`.
fn get_exponent(v: f32) -> i32 {
	let bits = v.to_bits() << 1 >> 1;
	(bits >> 23) as i32 - 127
}

/// Return the nearest integer to self. If a value is half-way between two integers, round to left.
///
/// # Examples
///
/// ```
/// # use rsr::pbrt::math::round_to_left;
/// assert_eq!(round_to_left(0.5), 0.);
/// assert_eq!(round_to_left(-0.5), -1.);
/// assert_eq!(round_to_left(0.4), 0.);
/// assert_eq!(round_to_left(-0.4), 0.);
/// assert_eq!(round_to_left(0.6), 1.);
/// assert_eq!(round_to_left(-0.6), -1.);
/// ```
pub fn round_to_left(v: Float) -> Float {
	if v.fract().abs() == 0.5 { v.floor() } else { v.round() }
}

/// Return the nearest integer to self. If a value is half-way between two integers, round to right.
///
/// # Examples
///
/// ```
/// # use rsr::pbrt::math::round_to_right;
/// assert_eq!(round_to_right(0.5), 1.);
/// assert_eq!(round_to_right(-0.5), 0.);
/// assert_eq!(round_to_right(0.4), 0.);
/// assert_eq!(round_to_right(-0.4), 0.);
/// assert_eq!(round_to_right(0.6), 1.);
/// assert_eq!(round_to_right(-0.6), -1.);
/// ```
pub fn round_to_right(v: Float) -> Float {
	if v.fract().abs() == 0.5 { v.ceil() } else { v.round() }
}

/// Usually return the index `i` such that `pred(i)` is true and `pred(i + 1)` is false, unless:
/// - The returned index `i` is no larger than `size - 2`.
/// - If there is no index such that the predicate is true, `0` is returned.
/// - If there is no index such that the predicate is false, `size - 2` is returned.
///
/// # Examples
///
/// ```
/// # use rsr::pbrt::math::find_interval;
/// assert_eq!(find_interval(4, |x| x < 3), 2);
/// assert_eq!(find_interval(4, |x| x > 5), 0);
/// assert_eq!(find_interval(4, |x| x < 5), 2);
/// ```
pub fn find_interval(size: usize, pred: impl Fn(usize) -> bool) -> usize {
	assert!(size >= 2);
	let mut l = 0;
	let mut r = size - 1;
	while l < r {
		let m = (l + r) / 2;
		if pred(m) {
			l = m + 1;
		} else {
			r = m;
		}
	}

	if l == size - 1 { size - 2 } else { l }
}

pub fn diff_of_products(a: Float, b: Float, c: Float, d: Float) -> Float {
	let cd = c * d;
	let result = a.mul_add(b, -cd);
	let error = (-c).mul_add(d, cd);
	result + error
}

pub fn sample_uniform_disk_concentric(u: Vector2f) -> Vector2f {
	let offset = 2. * u - Vector2f::new(1., 1.);
	// handle degeneracy at the origin
	if offset.x == 0. && offset.y == 0. {
		return Vector2f::default();
	}

	let (r, theta) = if offset.x.abs() > offset.y.abs() {
		(offset.x, FRAC_PI_4 * (offset.y / offset.x))
	} else {
		(offset.y, FRAC_PI_2 - FRAC_PI_4 * (offset.y / offset.x))
	};

	r * Vector2f::new(theta.cos(), theta.sin())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_exponent() {
		assert_eq!(get_exponent(2.0), 1);
		assert_eq!(get_exponent(-2.0), 1);
		assert_eq!(get_exponent(0.0), -127);
	}
}
