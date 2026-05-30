use super::{Float, vecmath::Vector3f};
use std::ops::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix44f(pub [[Float; 4]; 4]);

impl Matrix44f {
	pub fn has_nan(&self) -> bool {
		self.0.iter().flatten().any(|&i| i.is_nan())
	}

	/// Transpose
	#[allow(non_snake_case)]
	pub fn T(&self) -> Self {
		let mut ret = Matrix44f::default();
		for i in 0..4 {
			for j in 0..4 {
				ret[i][j] = self[j][i];
			}
		}

		ret
	}

	/// Multiply a point by this matrix, `p[3]`, i.e. w is treated as 1.0
	pub fn mul_point(&self, p: Vector3f) -> Vector3f {
		let mut ret = Vector3f::default();
		for i in 0..3 {
			for j in 0..3 {
				ret[i] += self[i][j] * p[j];
			}
			ret[i] += self[i][3];
		}
		let mut w = 0.0;
		for j in 0..3 {
			w += self[3][j] * p[j];
		}
		w += self[3][3];
		ret /= w;

		ret
	}
}

impl Index<usize> for Matrix44f {
	type Output = [Float; 4];

	fn index(&self, index: usize) -> &[Float; 4] {
		&self.0[index]
	}
}

impl IndexMut<usize> for Matrix44f {
	fn index_mut(&mut self, index: usize) -> &mut [Float; 4] {
		&mut self.0[index]
	}
}

impl Add for Matrix44f {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let mut ret = self;
		for i in 0..4 {
			for j in 0..4 {
				ret[i][j] += other[i][j];
			}
		}

		ret
	}
}

impl AddAssign for Matrix44f {
	fn add_assign(&mut self, other: Self) {
		for i in 0..4 {
			for j in 0..4 {
				self[i][j] += other[i][j];
			}
		}
	}
}

impl Sub for Matrix44f {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		let mut ret = self;
		for i in 0..4 {
			for j in 0..4 {
				ret[i][j] -= other[i][j];
			}
		}

		ret
	}
}

impl SubAssign for Matrix44f {
	fn sub_assign(&mut self, other: Self) {
		for i in 0..4 {
			for j in 0..4 {
				self[i][j] -= other[i][j];
			}
		}
	}
}

impl Mul for Matrix44f {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		let mut ret = Matrix44f::default();
		for i in 0..4 {
			for j in 0..4 {
				for k in 0..4 {
					ret[i][j] += self[i][k] * other[k][j];
				}
			}
		}

		ret
	}
}

impl MulAssign for Matrix44f {
	fn mul_assign(&mut self, other: Self) {
		*self = *self * other;
	}
}

impl Mul<Float> for Matrix44f {
	type Output = Self;

	fn mul(self, other: Float) -> Self {
		let mut ret = self;
		for i in 0..4 {
			for j in 0..4 {
				ret[i][j] *= other;
			}
		}

		ret
	}
}

impl MulAssign<Float> for Matrix44f {
	fn mul_assign(&mut self, other: Float) {
		for i in 0..4 {
			for j in 0..4 {
				self[i][j] *= other;
			}
		}
	}
}

impl Mul<Vector3f> for Matrix44f {
	type Output = Vector3f;

	fn mul(self, other: Vector3f) -> Vector3f {
		let mut ret = Vector3f::default();
		for i in 0..3 {
			for j in 0..3 {
				ret[i] += self[i][j] * other[j];
			}
		}

		ret
	}
}

impl Div<Float> for Matrix44f {
	type Output = Self;

	fn div(self, rhs: Float) -> Self {
		let inv = 1.0 / rhs;

		self * inv
	}
}

impl DivAssign<Float> for Matrix44f {
	fn div_assign(&mut self, rhs: Float) {
		let inv = 1.0 / rhs;
		*self *= inv;
	}
}
