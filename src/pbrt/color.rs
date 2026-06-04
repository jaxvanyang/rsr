use super::spectrum::{Spectrum, spectra};
use crate::{
	Float,
	pbrt::{Vector2f, spectrum::CIE_Y_INTEGRAL},
};
use std::ops;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct XYZ {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

impl XYZ {
	pub fn new(x: Float, y: Float, z: Float) -> Self {
		Self { x, y, z }
	}

	pub fn from_xyy(xy: Vector2f, y: Float) -> Self {
		if xy.y == 0. {
			Self::default()
		} else {
			Self::new(xy.x / xy.y * y, y, (1. - xy.x - xy.y) / xy.y * y)
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
	}
}
