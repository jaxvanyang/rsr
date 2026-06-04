use super::spectrum::{Spectrum, spectra};
use crate::{Float, pbrt::spectrum::CIE_Y_INTEGRAL};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct XYZ {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

impl XYZ {
	pub fn new(x: Float, y: Float, z: Float) -> Self {
		Self { x, y, z }
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

impl ops::Div<Float> for XYZ {
	type Output = Self;

	fn div(self, rhs: Float) -> Self {
		let inv = 1. / rhs;
		Self {
			x: self.x * inv,
			y: self.y * inv,
			z: self.z * inv,
		}
	}
}
