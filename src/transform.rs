use crate::{Float, Vector2f, Vector3f, diff_of_products};
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct SquareMatrix<const N: usize> {
	m: [[Float; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
	pub fn zero() -> Self {
		Self { m: [[0.0; N]; N] }
	}

	pub fn new(m: [[Float; N]; N]) -> Self {
		Self { m }
	}

	pub fn diag(values: [Float; N]) -> Self {
		let mut ret = Self::zero();
		for (i, row) in ret.m.iter_mut().enumerate() {
			row[i] = values[i];
		}
		ret
	}

	pub fn is_identity(&self) -> bool {
		for i in 0..N {
			for j in 0..N {
				if (i == j && self.m[i][j] != 1.0) || (i != j && self.m[i][j] != 0.0) {
					return false;
				}
			}
		}

		true
	}

	/// Return the transpose of this matrix.
	#[allow(non_snake_case)]
	pub fn T(&self) -> Self {
		let mut ret = Self::zero();
		for i in 0..N {
			for j in 0..N {
				ret.m[j][i] = self.m[i][j];
			}
		}
		ret
	}

	// TODO: implement linear_least_squares
}

impl SquareMatrix<1> {
	pub fn det(&self) -> Float {
		self[0][0]
	}

	pub fn inv(&self) -> Option<Self> {
		if self[0][0] == 0.0 {
			return None;
		}

		Some(Self {
			m: [[1.0 / self[0][0]]],
		})
	}
}

impl SquareMatrix<2> {
	pub fn det(&self) -> Float {
		diff_of_products(self[0][0], self[1][1], self[0][1], self[1][0])
	}

	pub fn inv(&self) -> Option<Self> {
		let det = self.det();
		if det == 0.0 {
			return None;
		}
		let inv_det = 1.0 / det;
		let mut ret = Self::zero();

		ret[0][0] = inv_det * self[1][1];
		ret[0][1] = inv_det * -self[0][1];
		ret[1][0] = inv_det * -self[1][0];
		ret[1][1] = inv_det * self[0][0];
		Some(ret)
	}
}

impl SquareMatrix<3> {
	pub fn det(&self) -> Float {
		let c00 = diff_of_products(self[1][1], self[2][2], self[1][2], self[2][1]);
		let c01 = diff_of_products(self[1][0], self[2][2], self[1][2], self[2][0]);
		let c02 = diff_of_products(self[1][0], self[2][1], self[1][1], self[2][0]);

		self[0][2].mul_add(c02, diff_of_products(self[0][0], c00, self[0][1], c01))
	}

	/// Return the inverse of this matrix, if it exists.
	pub fn inv(&self) -> Option<Self> {
		let det = self.det();
		if det == 0.0 {
			return None;
		}
		let inv_det = 1.0 / det;
		let mut ret = Self::zero();

		ret[0][0] = inv_det * diff_of_products(self[1][1], self[2][2], self[1][2], self[2][1]);
		ret[1][0] = inv_det * diff_of_products(self[1][2], self[2][0], self[1][0], self[2][2]);
		ret[2][0] = inv_det * diff_of_products(self[1][0], self[2][1], self[1][1], self[2][0]);
		ret[0][1] = inv_det * diff_of_products(self[0][2], self[2][1], self[0][1], self[2][2]);
		ret[1][1] = inv_det * diff_of_products(self[0][0], self[2][2], self[0][2], self[2][0]);
		ret[2][1] = inv_det * diff_of_products(self[0][1], self[2][0], self[0][0], self[2][1]);
		ret[0][2] = inv_det * diff_of_products(self[0][1], self[1][2], self[0][2], self[1][1]);
		ret[1][2] = inv_det * diff_of_products(self[0][2], self[1][0], self[0][0], self[1][2]);
		ret[2][2] = inv_det * diff_of_products(self[0][0], self[1][1], self[0][1], self[1][0]);

		Some(ret)
	}
}

impl SquareMatrix<4> {
	pub fn det(&self) -> Float {
		let s0 = diff_of_products(self[0][0], self[1][1], self[1][0], self[0][1]);
		let s1 = diff_of_products(self[0][0], self[1][2], self[1][0], self[0][2]);
		let s2 = diff_of_products(self[0][0], self[1][3], self[1][0], self[0][3]);

		let s3 = diff_of_products(self[0][1], self[1][2], self[1][1], self[0][2]);
		let s4 = diff_of_products(self[0][1], self[1][3], self[1][1], self[0][3]);
		let s5 = diff_of_products(self[0][2], self[1][3], self[1][2], self[0][3]);

		let c0 = diff_of_products(self[2][0], self[3][1], self[3][0], self[2][1]);
		let c1 = diff_of_products(self[2][0], self[3][2], self[3][0], self[2][2]);
		let c2 = diff_of_products(self[2][0], self[3][3], self[3][0], self[2][3]);

		let c3 = diff_of_products(self[2][1], self[3][2], self[3][1], self[2][2]);
		let c4 = diff_of_products(self[2][1], self[3][3], self[3][1], self[2][3]);
		let c5 = diff_of_products(self[2][2], self[3][3], self[3][2], self[2][3]);

		diff_of_products(s0, c5, s1, c4)
			+ diff_of_products(s2, c3, -s3, c2)
			+ diff_of_products(s5, c0, s4, c1)
	}

	/// Return the inverse of this matrix, if it exists.
	pub fn inv(&self) -> Option<Self> {
		// Via: https://github.com/google/ion/blob/master/ion/math/matrixutils.cc,
		// (c) Google, Apache license.

		// For 4x4 do not compute the adjugate as the transpose of the cofactor
		// matrix, because this results in extra work. Several calculations can be
		// shared across the sub-determinants.
		//
		// This approach is explained in David Eberly's Geometric Tools book,
		// excerpted here:
		//   http://www.geometrictools.com/Documentation/LaplaceExpansionTheorem.pdf
		let s0 = diff_of_products(self[0][0], self[1][1], self[1][0], self[0][1]);
		let s1 = diff_of_products(self[0][0], self[1][2], self[1][0], self[0][2]);
		let s2 = diff_of_products(self[0][0], self[1][3], self[1][0], self[0][3]);

		let s3 = diff_of_products(self[0][1], self[1][2], self[1][1], self[0][2]);
		let s4 = diff_of_products(self[0][1], self[1][3], self[1][1], self[0][3]);
		let s5 = diff_of_products(self[0][2], self[1][3], self[1][2], self[0][3]);

		let c0 = diff_of_products(self[2][0], self[3][1], self[3][0], self[2][1]);
		let c1 = diff_of_products(self[2][0], self[3][2], self[3][0], self[2][2]);
		let c2 = diff_of_products(self[2][0], self[3][3], self[3][0], self[2][3]);

		let c3 = diff_of_products(self[2][1], self[3][2], self[3][1], self[2][2]);
		let c4 = diff_of_products(self[2][1], self[3][3], self[3][1], self[2][3]);
		let c5 = diff_of_products(self[2][2], self[3][3], self[3][2], self[2][3]);

		// TODO: use EFT method
		let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 + s5 * c0 - s4 * c1;
		if det == 0.0 {
			return None;
		}
		let s = 1.0 / det;

		let inv = [
			[
				s * (self[1][1] * c5 + self[1][3] * c3 - self[1][2] * c4),
				s * (-self[0][1] * c5 + self[0][2] * c4 - self[0][3] * c3),
				s * (self[3][1] * s5 + self[3][3] * s3 - self[3][2] * s4),
				s * (-self[2][1] * s5 + self[2][2] * s4 - self[2][3] * s3),
			],
			[
				s * (-self[1][0] * c5 + self[1][2] * c2 - self[1][3] * c1),
				s * (self[0][0] * c5 + self[0][3] * c1 - self[0][2] * c2),
				s * (-self[3][0] * s5 + self[3][2] * s2 - self[3][3] * s1),
				s * (self[2][0] * s5 + self[2][3] * s1 - self[2][2] * s2),
			],
			[
				s * (self[1][0] * c4 + self[1][3] * c0 - self[1][1] * c2),
				s * (-self[0][0] * c4 + self[0][1] * c2 - self[0][3] * c0),
				s * (self[3][0] * s4 + self[3][3] * s0 - self[3][1] * s2),
				s * (-self[2][0] * s4 + self[2][1] * s2 - self[2][3] * s0),
			],
			[
				s * (-self[1][0] * c3 + self[1][1] * c1 - self[1][2] * c0),
				s * (self[0][0] * c3 + self[0][2] * c0 - self[0][1] * c1),
				s * (-self[3][0] * s3 + self[3][1] * s1 - self[3][2] * s0),
				s * (self[2][0] * s3 + self[2][2] * s0 - self[2][1] * s1),
			],
		];

		Some(Self { m: inv })
	}
}

impl<const N: usize> Default for SquareMatrix<N> {
	/// Return an identity matrix.
	fn default() -> Self {
		let mut ret = Self::zero();
		for i in 0..N {
			ret.m[i][i] = 1.0;
		}
		ret
	}
}

impl<const N: usize> ops::Add<&Self> for SquareMatrix<N> {
	type Output = Self;

	fn add(self, rhs: &Self) -> Self {
		let mut ret = self.clone();
		for i in 0..N {
			for j in 0..N {
				ret.m[i][j] += rhs.m[i][j];
			}
		}
		ret
	}
}

impl<const N: usize> ops::Mul<Float> for SquareMatrix<N> {
	type Output = Self;

	fn mul(self, rhs: Float) -> Self {
		let mut ret = self.clone();
		for i in 0..N {
			for j in 0..N {
				ret.m[i][j] *= rhs;
			}
		}
		ret
	}
}

impl<const N: usize> ops::Mul<&SquareMatrix<N>> for Float {
	type Output = SquareMatrix<N>;

	fn mul(self, rhs: &SquareMatrix<N>) -> SquareMatrix<N> {
		rhs.clone() * self
	}
}

impl<const N: usize> ops::Mul<&SquareMatrix<N>> for &SquareMatrix<N> {
	type Output = SquareMatrix<N>;

	fn mul(self, rhs: &SquareMatrix<N>) -> SquareMatrix<N> {
		let mut ret = SquareMatrix::zero();
		for i in 0..N {
			for j in 0..N {
				for k in 0..N {
					ret.m[i][j] += self.m[i][k] * rhs.m[k][j];
				}
			}
		}

		ret
	}
}

impl<const N: usize> ops::Div<Float> for SquareMatrix<N> {
	type Output = Self;

	fn div(self, rhs: Float) -> Self {
		debug_assert_ne!(rhs, 0.0);

		let mut ret = self.clone();
		for i in 0..N {
			for j in 0..N {
				ret.m[i][j] /= rhs;
			}
		}

		ret
	}
}

impl<const N: usize> ops::Index<usize> for SquareMatrix<N> {
	type Output = [Float; N];

	fn index(&self, index: usize) -> &[Float; N] {
		&self.m[index]
	}
}

impl<const N: usize> ops::IndexMut<usize> for SquareMatrix<N> {
	fn index_mut(&mut self, index: usize) -> &mut [Float; N] {
		&mut self.m[index]
	}
}

impl ops::Mul<Vector2f> for SquareMatrix<2> {
	type Output = Vector2f;

	fn mul(self, v: Vector2f) -> Vector2f {
		Vector2f::new(
			self.m[0][0] * v.x + self.m[0][1] * v.y,
			self.m[1][0] * v.x + self.m[1][1] * v.y,
		)
	}
}

impl ops::Mul<Vector3f> for SquareMatrix<3> {
	type Output = Vector3f;

	fn mul(self, v: Vector3f) -> Vector3f {
		Vector3f::new(
			self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
			self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
			self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_determinant() {
		let m = SquareMatrix::new([[1.0, 2.0], [3.0, 4.0]]);
		assert_eq!(m.det(), -2.0);

		let m = SquareMatrix::new([[0.0, 1.0, 2.0], [3.0, 0.0, 4.0], [5.0, 6.0, 0.0]]);
		assert_eq!(m.det(), 56.0);

		let m = SquareMatrix::new([
			[1.0, 2.0, 3.0, 0.0],
			[2.0, 6.0, 6.0, 1.0],
			[-1.0, 0.0, 0.0, 3.0],
			[0.0, 2.0, 0.0, 7.0],
		]);
		assert_eq!(m.det(), 36.0);

		let m = SquareMatrix::diag([1.0, 2.0, 3.0, 4.0]);
		assert_eq!(m.det(), 24.0);
	}

	#[test]
	fn test_inverse() {
		let m = SquareMatrix::new([[2.0]]);
		let inv = SquareMatrix::new([[0.5]]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());

		let m = SquareMatrix::new([[1.0, 2.0], [3.0, 4.0]]);
		let inv = SquareMatrix::new([[-2.0, 1.0], [1.5, -0.5]]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());

		let m = SquareMatrix::new([[2.0, 6.0, 2.0], [1.0, 4.0, 2.0], [5.0, 9.0, 0.0]]);
		let inv = SquareMatrix::new([[-9.0, 9.0, 2.0], [5.0, -5.0, -1.0], [-5.5, 6.0, 1.0]]);
		let singular = SquareMatrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());
		assert!(singular.inv().is_none());

		let m = SquareMatrix::new([
			[1.0, 1.0, 1.0, 1.0],
			[1.0, 1.0, -1.0, -1.0],
			[1.0, -1.0, -1.0, 1.0],
			[1.0, -1.0, 1.0, -1.0],
		]);
		let inv = SquareMatrix::new([
			[0.25, 0.25, 0.25, 0.25],
			[0.25, 0.25, -0.25, -0.25],
			[0.25, -0.25, -0.25, 0.25],
			[0.25, -0.25, 0.25, -0.25],
		]);
		let singular = SquareMatrix::new([
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 10.0, 11.0, 12.0],
			[13.0, 14.0, 15.0, 16.0],
		]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());
		assert!(singular.inv().is_none());
	}
}
