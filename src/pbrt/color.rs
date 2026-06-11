use super::{
	Float,
	math::{find_interval, lerp},
	number::Number,
	spectrum::{LAMBDA_MAX, LAMBDA_MIN, Spectrum, spectra},
	{Vector2f, spectrum::CIE_Y_INTEGRAL},
};
use crate::{polynomial, ui::Color};
use std::ops;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct XYZ {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

impl XYZ {
	// TODO: check NAN
	pub fn new(x: Float, y: Float, z: Float) -> Self {
		Self { x, y, z }
	}

	pub fn from_xy(xy: Vector2f) -> Self {
		Self::from_xyY(xy, 1.)
	}

	#[allow(non_snake_case)]
	pub fn from_xyY(xy: Vector2f, Y: Float) -> Self {
		if xy.y == 0. {
			Self::default()
		} else {
			Self::new(xy.x / xy.y * Y, Y, (1. - xy.x - xy.y) / xy.y * Y)
		}
	}

	pub fn xy(&self) -> Vector2f {
		Vector2f::new(self.x, self.y) / (self.x + self.y + self.z)
	}
}

impl From<&dyn Spectrum> for XYZ {
	fn from(s: &dyn Spectrum) -> Self {
		Self::new(
			spectra::x().inner_product(s),
			spectra::y().inner_product(s),
			spectra::z().inner_product(s),
		) / CIE_Y_INTEGRAL
	}
}

impl ops::Neg for XYZ {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			x: self.x,
			y: self.y,
			z: self.z,
		}
	}
}

impl ops::Add for XYZ {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret += rhs;
		ret
	}
}

impl ops::AddAssign for XYZ {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl ops::Sub for XYZ {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret -= rhs;
		ret
	}
}

impl ops::SubAssign for XYZ {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl ops::Sub<XYZ> for Float {
	type Output = XYZ;

	fn sub(self, rhs: XYZ) -> Self::Output {
		XYZ {
			x: self - rhs.x,
			y: self - rhs.y,
			z: self - rhs.z,
		}
	}
}

impl ops::Mul for XYZ {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret *= rhs;
		ret
	}
}

impl ops::MulAssign for XYZ {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}

impl ops::Mul<Float> for XYZ {
	type Output = Self;

	fn mul(self, rhs: Float) -> Self::Output {
		let mut ret = self;
		ret *= rhs;
		ret
	}
}

impl ops::MulAssign<Float> for XYZ {
	fn mul_assign(&mut self, rhs: Float) {
		debug_assert!(!rhs.is_nan());
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl ops::Div for XYZ {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret /= rhs;
		ret
	}
}

impl ops::DivAssign for XYZ {
	fn div_assign(&mut self, rhs: Self) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}

impl ops::Div<Float> for XYZ {
	type Output = Self;

	fn div(self, rhs: Float) -> Self::Output {
		let mut ret = self;
		ret /= rhs;
		ret
	}
}

impl ops::DivAssign<Float> for XYZ {
	fn div_assign(&mut self, rhs: Float) {
		debug_assert!(!rhs.is_nan());
		*self *= 1. / rhs;
	}
}

impl ops::Index<usize> for XYZ {
	type Output = Float;
	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("index out of bound: {index}"),
		}
	}
}

impl ops::IndexMut<usize> for XYZ {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			_ => panic!("index out of bound: {index}"),
		}
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RGB {
	pub r: Float,
	pub g: Float,
	pub b: Float,
}

impl RGB {
	pub const RED: Self = Self::new(1., 0., 0.);
	pub const GREEN: Self = Self::new(0., 1., 0.);
	pub const BLUE: Self = Self::new(0., 0., 1.);

	pub const fn new(r: Float, g: Float, b: Float) -> Self {
		Self { r, g, b }
	}

	pub fn avg(&self) -> Float {
		(self.r + self.g + self.b) / 3.
	}

	pub fn max_dimension(&self) -> usize {
		if self.r > self.g && self.r > self.b {
			0
		} else if self.g > self.b {
			1
		} else {
			2
		}
	}

	pub fn clamp(&self, min: Float, max: Float) -> Self {
		Self::new(
			self.r.clamp(min, max),
			self.g.clamp(min, max),
			self.b.clamp(min, max),
		)
	}

	pub fn clamp_zero(&self) -> Self {
		Self::new(self.r.max(0.), self.g.max(0.), self.b.max(0.))
	}
}

impl From<u32> for RGB {
	fn from(c: u32) -> Self {
		let r = c.r() as Float / 255.;
		let g = c.g() as Float / 255.;
		let b = c.b() as Float / 255.;

		Self { r, g, b }
	}
}

impl From<RGB> for u32 {
	fn from(c: RGB) -> Self {
		debug_assert!(0. <= c.r && c.r <= 1.);
		debug_assert!(0. <= c.g && c.g <= 1.);
		debug_assert!(0. <= c.b && c.b <= 1.);
		let r = (c.r * 255.) as u8;
		let g = (c.g * 255.) as u8;
		let b = (c.b * 255.) as u8;

		Self::from_rgb(r, g, b)
	}
}

impl ops::Neg for RGB {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			r: self.r,
			g: self.g,
			b: self.b,
		}
	}
}

impl ops::Add for RGB {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret += rhs;
		ret
	}
}

impl ops::AddAssign for RGB {
	fn add_assign(&mut self, rhs: Self) {
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
	}
}

impl ops::Sub for RGB {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret -= rhs;
		ret
	}
}

impl ops::SubAssign for RGB {
	fn sub_assign(&mut self, rhs: Self) {
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
	}
}

impl ops::Sub<RGB> for Float {
	type Output = RGB;

	fn sub(self, rhs: RGB) -> Self::Output {
		RGB {
			r: self - rhs.r,
			g: self - rhs.g,
			b: self - rhs.b,
		}
	}
}

impl ops::Mul for RGB {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret *= rhs;
		ret
	}
}

impl ops::MulAssign for RGB {
	fn mul_assign(&mut self, rhs: Self) {
		self.r *= rhs.r;
		self.g *= rhs.g;
		self.b *= rhs.b;
	}
}

impl ops::Mul<Float> for RGB {
	type Output = Self;

	fn mul(self, rhs: Float) -> Self::Output {
		let mut ret = self;
		ret *= rhs;
		ret
	}
}

impl ops::MulAssign<Float> for RGB {
	fn mul_assign(&mut self, rhs: Float) {
		debug_assert!(!rhs.is_nan());
		self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
	}
}

impl ops::Div for RGB {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		let mut ret = self;
		ret /= rhs;
		ret
	}
}

impl ops::DivAssign for RGB {
	fn div_assign(&mut self, rhs: Self) {
		self.r /= rhs.r;
		self.g /= rhs.g;
		self.b /= rhs.b;
	}
}

impl ops::Div<Float> for RGB {
	type Output = Self;

	fn div(self, rhs: Float) -> Self::Output {
		let mut ret = self;
		ret /= rhs;
		ret
	}
}

impl ops::DivAssign<Float> for RGB {
	fn div_assign(&mut self, rhs: Float) {
		debug_assert!(!rhs.is_nan());
		*self *= 1. / rhs;
	}
}

impl ops::Index<usize> for RGB {
	type Output = Float;
	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.r,
			1 => &self.g,
			2 => &self.b,
			_ => panic!("index out of bound: {index}"),
		}
	}
}

impl ops::IndexMut<usize> for RGB {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.r,
			1 => &mut self.g,
			2 => &mut self.b,
			_ => panic!("index out of bound: {index}"),
		}
	}
}

#[derive(Debug)]
pub struct RGBSigmoidPolynomial {
	c0: Float,
	c1: Float,
	c2: Float,
}

impl RGBSigmoidPolynomial {
	/// Create a function: `sigmoid(c0 * x^2 + c1 * x + c2)`.
	pub fn new(c0: Float, c1: Float, c2: Float) -> Self {
		Self { c0, c1, c2 }
	}

	pub fn eval(&self, lambda: Float) -> Float {
		Self::s(polynomial!(lambda, self.c2, self.c1, self.c0))
	}

	pub fn max_value(&self) -> Float {
		let mut ret = self.eval(LAMBDA_MIN).max(self.eval(LAMBDA_MAX));
		let lambda = -self.c1 / (2. * self.c0);
		if LAMBDA_MIN < lambda && lambda < LAMBDA_MAX {
			ret = ret.max(self.eval(lambda));
		}

		ret
	}

	fn s(x: Float) -> Float {
		if x.is_infinite() {
			if x > 0. { 1. } else { 0. }
		} else {
			0.5 + x / (2. * (1. + x.powi(2)).sqrt())
		}
	}
}

/// Resolution of `RGBToSpectrumTable`.
const RES: usize = 64;
type CoefficientArray = [[[[[f32; 3]; RES]; RES]; RES]; 3];

#[link(name = "pbrt_data")]
unsafe extern "C" {
	static ACES2065_1ToSpectrumTable_Scale: [f32; RES];
	static ACES2065_1ToSpectrumTable_Data: CoefficientArray;
	static DCI_P3ToSpectrumTable_Scale: [f32; RES];
	static DCI_P3ToSpectrumTable_Data: CoefficientArray;
	static REC2020ToSpectrumTable_Scale: [f32; RES];
	static REC2020ToSpectrumTable_Data: CoefficientArray;
	static sRGBToSpectrumTable_Scale: [f32; RES];
	static sRGBToSpectrumTable_Data: CoefficientArray;
}

#[derive(Debug, PartialEq)]
pub struct RGBToSpectrumTable<'a> {
	z_nodes: &'a [f32; RES],
	coeffs: &'a CoefficientArray,
}

impl<'a> RGBToSpectrumTable<'a> {
	#[allow(non_upper_case_globals)]
	pub const sRGB: Self =
		unsafe { Self::new(&sRGBToSpectrumTable_Scale, &sRGBToSpectrumTable_Data) };
	pub const DCI_P3: Self =
		unsafe { Self::new(&DCI_P3ToSpectrumTable_Scale, &DCI_P3ToSpectrumTable_Data) };
	#[allow(non_upper_case_globals)]
	pub const Rec2020: Self =
		unsafe { Self::new(&REC2020ToSpectrumTable_Scale, &REC2020ToSpectrumTable_Data) };
	pub const ACES2065_1: Self = unsafe {
		Self::new(
			&ACES2065_1ToSpectrumTable_Scale,
			&ACES2065_1ToSpectrumTable_Data,
		)
	};

	pub const fn new(z_nodes: &'a [f32; RES], coeffs: &'a CoefficientArray) -> Self {
		Self { z_nodes, coeffs }
	}

	pub fn eval(&self, rgb: RGB) -> RGBSigmoidPolynomial {
		// handle uniform RGB values
		if rgb.r == rgb.g && rgb.g == rgb.b {
			return RGBSigmoidPolynomial::new(
				0.,
				0.,
				(rgb.r - 0.5) / (rgb.r * (1. - rgb.r)).sqrt(),
			);
		}

		// find maximum component and compute remapped component values
		let maxc = rgb.max_dimension();
		let z = rgb[maxc];
		let x = rgb[(maxc + 1) % 3] * (RES as Float - 1.) / z;
		let y = rgb[(maxc + 2) % 3] * (RES as Float - 1.) / z;
		// compute integer indices and offsets for coefficient interpolation
		let xi = (x as usize).min(RES - 2);
		let yi = (y as usize).min(RES - 2);
		let zi = find_interval(RES, |i| self.z_nodes[i].as_float() < z);
		let dx = x - xi as Float;
		let dy = y - yi as Float;
		let dz = (z - self.z_nodes[zi].as_float())
			/ (self.z_nodes[zi + 1] - self.z_nodes[zi]).as_float();
		// trilinearly interpolate sigmoid polynomial coefficients c
		let mut c = [0.; 3];
		for (i, ci) in c.iter_mut().enumerate() {
			let co = |dx: usize, dy: usize, dz: usize| {
				self.coeffs[maxc][zi + dz][yi + dy][xi + dx][i].as_float()
			};
			*ci = lerp(
				dz,
				lerp(
					dy,
					lerp(dx, co(0, 0, 0), co(1, 0, 0)),
					lerp(dx, co(0, 1, 0), co(1, 1, 0)),
				),
				lerp(
					dy,
					lerp(dx, co(0, 0, 1), co(1, 0, 1)),
					lerp(dx, co(0, 1, 1), co(1, 1, 1)),
				),
			);
		}

		RGBSigmoidPolynomial::new(c[0], c[1], c[2])
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_xyz_arithmetic() {
		let a = XYZ::new(1., 2., 3.);
		let b = XYZ::new(4., 5., 6.);
		assert_eq!(a + b, XYZ::new(5., 7., 9.));
		assert_eq!(a - b, XYZ::new(-3., -3., -3.));
		assert_eq!(a * b, XYZ::new(4., 10., 18.));
		assert_eq!(a / b, XYZ::new(0.25, 0.4, 0.5));
		assert_eq!(a * 2., XYZ::new(2., 4., 6.));
		assert_eq!(a / 2., XYZ::new(0.5, 1., 1.5));
		assert_eq!(1. - a, XYZ::new(0., -1., -2.));
	}

	#[test]
	fn test_rgb_arithmetic() {
		let a = RGB::new(1., 2., 3.);
		let b = RGB::new(4., 5., 6.);
		assert_eq!(a + b, RGB::new(5., 7., 9.));
		assert_eq!(a - b, RGB::new(-3., -3., -3.));
		assert_eq!(a * b, RGB::new(4., 10., 18.));
		assert_eq!(a / b, RGB::new(0.25, 0.4, 0.5));
		assert_eq!(a * 2., RGB::new(2., 4., 6.));
		assert_eq!(a / 2., RGB::new(0.5, 1., 1.5));
		assert_eq!(1. - a, RGB::new(0., -1., -2.));
	}
}
