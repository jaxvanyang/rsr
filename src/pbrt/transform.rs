use super::{
	Float, diff_of_products,
	vecmath::{Bounds3f, Vector2f, Vector3f},
};
use approx::abs_diff_eq;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct SquareMatrix<const N: usize> {
	m: [[Float; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
	pub fn zero() -> Self {
		Self { m: [[0.0; N]; N] }
	}

	pub fn has_nan(&self) -> bool {
		self.m.iter().flatten().any(|x| x.is_nan())
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

	pub fn mul_point(&self, p: Vector3f) -> Vector3f {
		let m = &self.m;
		let x = m[0][0] * p.x + m[0][1] * p.y + m[0][2] * p.z + m[0][3];
		let y = m[1][0] * p.x + m[1][1] * p.y + m[1][2] * p.z + m[1][3];
		let z = m[2][0] * p.x + m[2][1] * p.y + m[2][2] * p.z + m[2][3];
		let w = m[3][0] * p.x + m[3][1] * p.y + m[3][2] * p.z + m[3][3];

		if w == 1.0 {
			Vector3f::new(x, y, z)
		} else {
			Vector3f::new(x, y, z) / w
		}
	}

	pub fn mul_vector(&self, v: Vector3f) -> Vector3f {
		let m = &self.m;
		let x = m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z;
		let y = m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z;
		let z = m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z;

		Vector3f::new(x, y, z)
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

impl<const N: usize> From<[[Float; N]; N]> for SquareMatrix<N> {
	fn from(m: [[Float; N]; N]) -> Self {
		let ret = Self { m };
		debug_assert!(!ret.has_nan());

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

impl<const N: usize> ops::Mul<&SquareMatrix<N>> for SquareMatrix<N> {
	type Output = SquareMatrix<N>;

	fn mul(self, rhs: &SquareMatrix<N>) -> SquareMatrix<N> {
		(&self) * rhs
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

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
	m: SquareMatrix<4>,
	inv: Option<SquareMatrix<4>>,
}

impl Transform {
	pub fn new(m: SquareMatrix<4>) -> Self {
		let inv = m.inv();

		Self { m, inv }
	}

	pub fn new_with_inv(m: SquareMatrix<4>, inv: SquareMatrix<4>) -> Self {
		Self { m, inv: Some(inv) }
	}

	pub fn get_matrix(&self) -> &SquareMatrix<4> {
		&self.m
	}

	pub fn get_inverse_matrix(&self) -> Option<&SquareMatrix<4>> {
		self.inv.as_ref()
	}

	/// Return the inverse of this transform, if it exists.
	pub fn inv(&self) -> Option<Self> {
		self.inv
			.as_ref()
			.map(|inv| Self::new_with_inv(inv.clone(), self.m.clone()))
	}

	/// Return the transpose of this transform.
	#[allow(non_snake_case)]
	pub fn T(&self) -> Self {
		Self {
			m: self.m.T(),
			inv: self.inv.as_ref().map(|inv| inv.T()),
		}
	}

	pub fn is_identity(&self) -> bool {
		self.m.is_identity()
	}

	pub fn translate(delta: Vector3f) -> Self {
		let m = SquareMatrix::from([
			[1.0, 0.0, 0.0, delta.x],
			[0.0, 1.0, 0.0, delta.y],
			[0.0, 0.0, 1.0, delta.z],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let inv = SquareMatrix::from([
			[1.0, 0.0, 0.0, -delta.x],
			[0.0, 1.0, 0.0, -delta.y],
			[0.0, 0.0, 1.0, -delta.z],
			[0.0, 0.0, 0.0, 1.0],
		]);

		Self { m, inv: Some(inv) }
	}

	pub fn scale(x: Float, y: Float, z: Float) -> Self {
		let m = SquareMatrix::from([
			[x, 0.0, 0.0, 0.0],
			[0.0, y, 0.0, 0.0],
			[0.0, 0.0, z, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let inv = SquareMatrix::from([
			[1.0 / x, 0.0, 0.0, 0.0],
			[0.0, 1.0 / y, 0.0, 0.0],
			[0.0, 0.0, 1.0 / z, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);

		Self { m, inv: Some(inv) }
	}

	/// Return `true` if the transform has a scaling term.
	pub fn has_scale(&self) -> bool {
		let tolerance = 1e-3;
		let lx2 = self
			.map_point(Vector3f::new(1.0, 0.0, 0.0))
			.length_squared();
		let ly2 = self
			.map_point(Vector3f::new(0.0, 1.0, 0.0))
			.length_squared();
		let lz2 = self
			.map_point(Vector3f::new(0.0, 0.0, 1.0))
			.length_squared();

		(lx2 - 1.0).abs() > tolerance
			|| (ly2 - 1.0).abs() > tolerance
			|| (lz2 - 1.0).abs() > tolerance
	}

	/// Return a rotation transform around the x-axis, `theta` is in degrees.
	pub fn rotate_x(theta: Float) -> Self {
		let theta = theta.to_radians();
		let sin = theta.sin();
		let cos = theta.cos();
		let m = SquareMatrix::from([
			[1.0, 0.0, 0.0, 0.0],
			[0.0, cos, -sin, 0.0],
			[0.0, sin, cos, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let inv = m.T();

		Self { m, inv: Some(inv) }
	}

	/// Return a rotation transform around the y-axis, `theta` is in degrees.
	pub fn rotate_y(theta: Float) -> Self {
		let theta = theta.to_radians();
		let sin = theta.sin();
		let cos = theta.cos();
		let m = SquareMatrix::from([
			[cos, 0.0, sin, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[-sin, 0.0, cos, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let inv = m.T();

		Self { m, inv: Some(inv) }
	}

	/// Return a rotation transform around the y-axis, `theta` is in degrees.
	pub fn rotate_z(theta: Float) -> Self {
		let theta = theta.to_radians();
		let sin = theta.sin();
		let cos = theta.cos();
		let m = SquareMatrix::from([
			[cos, -sin, 0.0, 0.0],
			[sin, cos, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let inv = m.T();

		Self { m, inv: Some(inv) }
	}

	/// Return a rotation transform around `axis`, `theta` is in degrees.
	pub fn rotate(axis: Vector3f, theta: Float) -> Self {
		let theta = theta.to_radians();
		Self::rotate_with_sin_cos(axis, theta.sin(), theta.cos())
	}

	/// Return a rotation transform around `axis` with precomputed `sin(theta)` and `cos(theta)` values.
	pub fn rotate_with_sin_cos(axis: Vector3f, sin: Float, cos: Float) -> Self {
		let a = axis.normalized();
		let mut m = SquareMatrix::zero();
		m[3][3] = 1.0;

		// TODO: why does it says the row vectors to be basis
		// Compute rotation of first basis vector
		m[0][0] = a.x * a.x + (1.0 - a.x * a.x) * cos;
		m[0][1] = a.x * a.y * (1.0 - cos) - a.z * sin;
		m[0][2] = a.x * a.z * (1.0 - cos) + a.y * sin;

		// Compute rotations of second and third basis vectors
		m[1][0] = a.x * a.y * (1.0 - cos) + a.z * sin;
		m[1][1] = a.y * a.y + (1.0 - a.y * a.y) * cos;
		m[1][2] = a.y * a.z * (1.0 - cos) - a.x * sin;

		m[2][0] = a.x * a.z * (1.0 - cos) - a.y * sin;
		m[2][1] = a.y * a.z * (1.0 - cos) + a.x * sin;
		m[2][2] = a.z * a.z + (1.0 - a.z * a.z) * cos;

		let inv = m.T();

		Self { m, inv: Some(inv) }
	}

	/// Returns a rotation matrix that rotates from one vector to another. The arguments must be normalized.
	/// Note: the result transform may not be the shortest rotation.
	pub fn rotate_from_to(from: Vector3f, to: Vector3f) -> Self {
		// Compute intermediate vector for vector reflection
		let refl = if from.x.abs() < 0.72 && to.x.abs() < 0.72 {
			Vector3f::new(1.0, 0.0, 0.0)
		} else if from.y.abs() < 0.72 && to.y.abs() < 0.72 {
			Vector3f::new(0.0, 1.0, 0.0)
		} else {
			Vector3f::new(0.0, 0.0, 1.0)
		};

		// TODO: understand Householder matrix
		// Initialize matrix r for rotation
		let u = refl - from;
		let v = refl - to;
		let mut r = SquareMatrix::zero();
		r[3][3] = 1.0;
		for i in 0..3 {
			for j in 0..3 {
				// Initialize matrix element `r[i][j]`
				let delta = if i == j { 1.0 } else { 0.0 };
				r[i][j] = delta
					- 2.0 / u.length_squared() * u[i] * u[j]
					- 2.0 / v.length_squared() * v[i] * v[j]
					+ 4.0 * u.dot(v) / (u.length_squared() * v.length_squared()) * v[i] * u[j];
			}
		}
		let inv = r.T();

		Self {
			m: r,
			inv: Some(inv),
		}
	}

	/// Create a look-at transform to camera space, the camera is looking at `look` from
	/// `pos`, with `up` as the up direction. `up` may not be perpendicular to `look`
	/// direction.
	pub fn look_at(pos: Vector3f, look: Vector3f, up: Vector3f) -> Self {
		let mut camera_to_world = SquareMatrix::<4>::zero();
		camera_to_world[0][3] = pos.x;
		camera_to_world[1][3] = pos.y;
		camera_to_world[2][3] = pos.z;
		camera_to_world[3][3] = 1.0;

		// TODO: check `up` direction
		let dir = (look - pos).normalized();
		let right = up.normalized().cross(dir).normalized();
		let up = dir.cross(right);
		camera_to_world[0][0] = right.x;
		camera_to_world[1][0] = right.y;
		camera_to_world[2][0] = right.z;
		camera_to_world[0][1] = up.x;
		camera_to_world[1][1] = up.y;
		camera_to_world[2][1] = up.z;
		camera_to_world[0][2] = dir.x;
		camera_to_world[1][2] = dir.y;
		camera_to_world[2][2] = dir.z;

		let world_to_camera = camera_to_world.inv().unwrap();

		Self {
			m: world_to_camera,
			inv: Some(camera_to_world),
		}
	}

	/// Apply the transform to a point.
	pub fn map_point(&self, p: Vector3f) -> Vector3f {
		self.m.mul_point(p)
	}

	/// Apply the inverse transform to a point.
	pub fn invert_point(&self, p: Vector3f) -> Option<Vector3f> {
		self.inv.as_ref().map(|m| m.mul_point(p))
	}

	/// Apply the transform to a vector.
	pub fn map_vector(&self, v: Vector3f) -> Vector3f {
		self.m.mul_vector(v)
	}

	/// Apply the inverse transform to a vector.
	pub fn invert_vector(&self, v: Vector3f) -> Option<Vector3f> {
		self.inv.as_ref().map(|m| m.mul_vector(v))
	}

	/// Apply the transform to a normal.
	pub fn map_normal(&self, n: Vector3f) -> Option<Vector3f> {
		let m = self.inv.as_ref()?;
		let x = m[0][0] * n.x + m[1][0] * n.y + m[2][0] * n.z;
		let y = m[0][1] * n.x + m[1][1] * n.y + m[2][1] * n.z;
		let z = m[0][2] * n.x + m[1][2] * n.y + m[2][2] * n.z;

		Some(Vector3f::new(x, y, z))
	}

	/// Apply the inverse transform to a normal.
	pub fn invert_normal(&self, n: Vector3f) -> Vector3f {
		let m = &self.m;
		let x = m[0][0] * n.x + m[1][0] * n.y + m[2][0] * n.z;
		let y = m[0][1] * n.x + m[1][1] * n.y + m[2][1] * n.z;
		let z = m[0][2] * n.x + m[1][2] * n.y + m[2][2] * n.z;

		Vector3f::new(x, y, z)
	}

	// TODO: transform ray

	/// Apply the transform to a bounding box.
	pub fn map_bounds(&self, b: &Bounds3f) -> Bounds3f {
		Bounds3f::from_point(self.map_point(b.corner(0)))
			.union_point(self.map_point(b.corner(1)))
			.union_point(self.map_point(b.corner(2)))
			.union_point(self.map_point(b.corner(4)))
	}

	/// Apply the inverse transform to a bounding box.
	pub fn invert_bounds(&self, b: &Bounds3f) -> Option<Bounds3f> {
		let m = self.inv.as_ref()?;
		let ret = Bounds3f::from_point(m.mul_point(b.corner(0)))
			.union_point(m.mul_point(b.corner(1)))
			.union_point(m.mul_point(b.corner(2)))
			.union_point(m.mul_point(b.corner(4)));

		Some(ret)
	}

	/// Return `true` if the transform changes handedness.
	pub fn swaps_handedness(&self) -> bool {
		let m = &self.m;
		let m = SquareMatrix::from([
			[m[0][0], m[0][1], m[0][2]],
			[m[1][0], m[1][1], m[1][2]],
			[m[2][0], m[2][1], m[2][2]],
		]);

		m.det() < 0.0
	}
}

impl Default for Transform {
	fn default() -> Self {
		let m = SquareMatrix::default();
		Self {
			m: m.clone(),
			inv: Some(m),
		}
	}
}

impl From<[[Float; 4]; 4]> for Transform {
	fn from(m: [[Float; 4]; 4]) -> Self {
		let m = SquareMatrix::from(m);
		let inv = m.inv();

		Self { m, inv }
	}
}

impl From<&Frame> for Transform {
	fn from(f: &Frame) -> Self {
		let m = SquareMatrix::from([
			[f.x.x, f.x.y, f.x.z, 0.0],
			[f.y.x, f.y.y, f.y.z, 0.0],
			[f.z.x, f.z.y, f.z.z, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let inv = m.T();

		Self { m, inv: Some(inv) }
	}
}

impl ops::Mul<&Transform> for Transform {
	type Output = Transform;

	fn mul(self, rhs: &Transform) -> Self::Output {
		(&self) * rhs
	}
}

impl ops::Mul<&Transform> for &Transform {
	type Output = Transform;

	fn mul(self, rhs: &Transform) -> Self::Output {
		let m = &self.m * &rhs.m;
		let inv = if let Some(a) = self.inv.as_ref()
			&& let Some(b) = rhs.inv.as_ref()
		{
			Some(b * a)
		} else {
			None
		};

		Self::Output { m, inv }
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
	pub x: Vector3f,
	pub y: Vector3f,
	pub z: Vector3f,
}

impl Frame {
	pub fn new(x: Vector3f, y: Vector3f, z: Vector3f) -> Self {
		let ret = Self { x, y, z };

		debug_assert!(x.is_normalized());
		debug_assert!(y.is_normalized());
		debug_assert!(z.is_normalized());
		debug_assert!(abs_diff_eq!(x.cross(y), z));

		ret
	}

	pub fn from_xz(x: Vector3f, z: Vector3f) -> Self {
		Self::new(x, z.cross(x), z)
	}

	pub fn from_xy(x: Vector3f, y: Vector3f) -> Self {
		Self::new(x, y, x.cross(y))
	}

	pub fn from_z(z: Vector3f) -> Self {
		let (x, y) = z.coordinate_system();

		Self::new(x, y, z)
	}

	/// Transform a vector / normal into the frame's local coordinate system.
	pub fn to_local(&self, v: Vector3f) -> Vector3f {
		Vector3f::new(self.x.dot(v), self.y.dot(v), self.z.dot(v))
	}

	/// Transform a vector / normal from the frame's local coordinate system to world space.
	pub fn from_local(&self, v: Vector3f) -> Vector3f {
		v.x * self.x + v.y * self.y + v.z * self.z
	}
}

impl Default for Frame {
	fn default() -> Self {
		Self {
			x: Vector3f::new(1.0, 0.0, 0.0),
			y: Vector3f::new(0.0, 1.0, 0.0),
			z: Vector3f::new(0.0, 0.0, 1.0),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;

	#[test]
	fn test_determinant() {
		let m = SquareMatrix::from([[1.0, 2.0], [3.0, 4.0]]);
		assert_eq!(m.det(), -2.0);

		let m = SquareMatrix::from([[0.0, 1.0, 2.0], [3.0, 0.0, 4.0], [5.0, 6.0, 0.0]]);
		assert_eq!(m.det(), 56.0);

		let m = SquareMatrix::from([
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
		let m = SquareMatrix::from([[2.0]]);
		let inv = SquareMatrix::from([[0.5]]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());

		let m = SquareMatrix::from([[1.0, 2.0], [3.0, 4.0]]);
		let inv = SquareMatrix::from([[-2.0, 1.0], [1.5, -0.5]]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());

		let m = SquareMatrix::from([[2.0, 6.0, 2.0], [1.0, 4.0, 2.0], [5.0, 9.0, 0.0]]);
		let inv = SquareMatrix::from([[-9.0, 9.0, 2.0], [5.0, -5.0, -1.0], [-5.5, 6.0, 1.0]]);
		let singular = SquareMatrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());
		assert!(singular.inv().is_none());

		let m = SquareMatrix::from([
			[1.0, 1.0, 1.0, 1.0],
			[1.0, 1.0, -1.0, -1.0],
			[1.0, -1.0, -1.0, 1.0],
			[1.0, -1.0, 1.0, -1.0],
		]);
		let inv = SquareMatrix::from([
			[0.25, 0.25, 0.25, 0.25],
			[0.25, 0.25, -0.25, -0.25],
			[0.25, -0.25, -0.25, 0.25],
			[0.25, -0.25, 0.25, -0.25],
		]);
		let singular = SquareMatrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 10.0, 11.0, 12.0],
			[13.0, 14.0, 15.0, 16.0],
		]);
		assert_eq!(m.inv().unwrap(), inv);
		assert!((&m * &inv).is_identity());
		assert!(singular.inv().is_none());
	}

	#[test]
	fn test_translate() {
		let t = Transform::translate(Vector3f::new(1.0, 2.0, 3.0));
		let p = Vector3f::new(4.0, 5.0, 6.0);
		let q = Vector3f::new(5.0, 7.0, 9.0);
		assert_eq!(t.map_point(p), q);
		assert_eq!(t.invert_point(q).unwrap(), p);
		assert_eq!(t.map_vector(p), p);
		assert_eq!(t.invert_vector(p).unwrap(), p);
		assert_eq!(t.map_normal(p).unwrap(), p);
		assert_eq!(t.invert_normal(p), p);
	}

	#[test]
	fn test_scale() {
		let t = Transform::scale(1.0, 2.0, 3.0);
		let p = Vector3f::new(4.0, 5.0, 6.0);
		let q = Vector3f::new(4.0, 10.0, 18.0);
		let n = Vector3f::new(4.0, 2.5, 2.0);
		assert!(t.has_scale());
		assert_eq!(t.map_point(p), q);
		assert_eq!(t.invert_point(q).unwrap(), p);
		assert_eq!(t.map_vector(p), q);
		assert_eq!(t.invert_vector(q).unwrap(), p);
		assert_eq!(t.map_normal(p).unwrap(), n);
		assert_eq!(t.invert_normal(n), p);

		let t = Transform::default();
		assert!(t.is_identity());
		assert!(!t.has_scale());
	}

	#[test]
	fn test_rotate() {
		let t = Transform::rotate_x(90.0);
		let p = Vector3f::new(1.0, 2.0, 3.0);
		let q = Vector3f::new(1.0, -3.0, 2.0);
		assert_abs_diff_eq!(t.map_point(p), q);
		assert_abs_diff_eq!(t.invert_point(q).unwrap(), p, epsilon = 1e-6);
		assert_abs_diff_eq!(t.map_vector(p), q);
		assert_abs_diff_eq!(t.invert_vector(q).unwrap(), p, epsilon = 1e-6);
		assert_abs_diff_eq!(t.map_normal(p).unwrap(), q);
		assert_abs_diff_eq!(t.invert_normal(q), p, epsilon = 1e-6);

		let t = Transform::rotate_y(90.0);
		let p = Vector3f::new(1.0, 2.0, 3.0);
		let q = Vector3f::new(3.0, 2.0, -1.0);
		assert_abs_diff_eq!(t.map_point(p), q);
		assert_abs_diff_eq!(t.invert_point(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_vector(p), q);
		assert_abs_diff_eq!(t.invert_vector(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_normal(p).unwrap(), q);
		assert_abs_diff_eq!(t.invert_normal(q), p);

		let t = Transform::rotate_z(90.0);
		let p = Vector3f::new(1.0, 2.0, 3.0);
		let q = Vector3f::new(-2.0, 1.0, 3.0);
		assert_abs_diff_eq!(t.map_point(p), q);
		assert_abs_diff_eq!(t.invert_point(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_vector(p), q);
		assert_abs_diff_eq!(t.invert_vector(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_normal(p).unwrap(), q);
		assert_abs_diff_eq!(t.invert_normal(q), p);

		let t = Transform::rotate(Vector3f::new(1.0, 1.0, 1.0), 120.0);
		let p = Vector3f::new(1.0, 0.0, 0.0);
		let q = Vector3f::new(0.0, 1.0, 0.0);
		assert_abs_diff_eq!(t.map_point(p), q, epsilon = 1e-6);
		assert_abs_diff_eq!(t.invert_point(q).unwrap(), p, epsilon = 1e-6);
		assert_abs_diff_eq!(t.map_vector(p), q, epsilon = 1e-6);
		assert_abs_diff_eq!(t.invert_vector(q).unwrap(), p, epsilon = 1e-6);
		assert_abs_diff_eq!(t.map_normal(p).unwrap(), q, epsilon = 1e-6);
		assert_abs_diff_eq!(t.invert_normal(q), p, epsilon = 1e-6);

		// the rotation axis is (1, 1, 1)
		let t =
			Transform::rotate_from_to(Vector3f::new(1.0, 0.0, 0.0), Vector3f::new(0.0, 1.0, 0.0));
		let p = Vector3f::new(1.0, 1.0, 0.0);
		let q = Vector3f::new(0.0, 1.0, 1.0);
		assert_eq!(t.map_point(p), q);
		assert_eq!(t.invert_point(q).unwrap(), p);
		assert_eq!(t.map_vector(p), q);
		assert_eq!(t.invert_vector(q).unwrap(), p);
		assert_eq!(t.map_normal(p).unwrap(), q);
		assert_eq!(t.invert_normal(q), p);
	}

	#[test]
	fn test_look_at() {
		let t = Transform::look_at(
			Vector3f::new(2.0, 2.0, 2.0),
			Vector3f::new(0.0, 0.0, 0.0),
			Vector3f::new(-1.0, 0.0, -1.0),
		);
		let p = Vector3f::new(1.0, 1.0, 1.0);
		let q = Vector3f::new(0.0, 0.0, (3.0 as Float).sqrt());
		let v = Vector3f::new(0.0, 0.0, -(3.0 as Float).sqrt());
		assert_abs_diff_eq!(t.map_point(p), q, epsilon = 1e-6);
		assert_abs_diff_eq!(t.invert_point(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_vector(p), v);
		assert_abs_diff_eq!(t.invert_vector(v).unwrap(), p);
		assert_abs_diff_eq!(t.map_normal(p).unwrap(), v, epsilon = 1e-6);
		assert_abs_diff_eq!(t.invert_normal(v), p);
	}

	#[test]
	fn test_bounds() {
		let t = Transform::scale(-1.0, 2.0, -3.0);
		let b = Bounds3f::new(
			Vector3f::new(-1.0, -1.0, -1.0),
			Vector3f::new(1.0, 1.0, 1.0),
		);
		let c = Bounds3f::new(
			Vector3f::new(-1.0, -2.0, -3.0),
			Vector3f::new(1.0, 2.0, 3.0),
		);
		assert_eq!(t.map_bounds(&b), c);
		assert_eq!(t.invert_bounds(&c).unwrap(), b);
	}

	#[test]
	fn test_composition() {
		let tx = Transform::rotate_x(90.0);
		let ty = Transform::rotate_y(90.0);
		let tz = Transform::rotate_z(90.0);
		let t = &tz * &ty * &tx;
		let p = Vector3f::new(1.0, 1.0, 1.0);
		let q = Vector3f::new(1.0, 1.0, -1.0);
		assert_abs_diff_eq!(t.map_point(p), q);
		assert_abs_diff_eq!(t.invert_point(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_vector(p), q);
		assert_abs_diff_eq!(t.invert_vector(q).unwrap(), p);
		assert_abs_diff_eq!(t.map_normal(p).unwrap(), q);
		assert_abs_diff_eq!(t.invert_normal(q), p);
	}

	#[test]
	fn test_frame() {
		let f = Frame::from_z(Vector3f::new(1.0, 0.0, 0.0));
		let t = Transform::from(&f);
		let u = Vector3f::new(1.0, 2.0, 3.0);
		let v = Vector3f::new(-3.0, 2.0, 1.0);
		assert_eq!(f.to_local(u), v);
		assert_eq!(f.from_local(v), u);
		assert_eq!(t.map_vector(u), v);
		assert_eq!(t.invert_vector(v).unwrap(), u);
	}
}
