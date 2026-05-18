use crate::{Float, Number, lerp};
use approx::{AbsDiffEq, abs_diff_eq};
use std::fmt::Display;
use std::ops::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

pub type Vector2f = Vector2<Float>;
pub type Vector2i = Vector2<i32>;

impl<T: Number> Vector2<T> {
	pub fn new(x: T, y: T) -> Self {
		let ret = Self { x, y };
		debug_assert!(!ret.has_nan());
		ret
	}

	pub fn has_nan(self) -> bool {
		self.x.is_nan() || self.y.is_nan()
	}

	pub fn abs(self) -> Self {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
		}
	}

	pub fn length_squared(self) -> T {
		self.dot(self)
	}

	pub fn abs_dot(&self, rhs: Self) -> T {
		self.dot(rhs).abs()
	}

	pub fn dot(&self, rhs: Self) -> T {
		self.x * rhs.x + self.y * rhs.y
	}

	/// per element min
	pub fn min(self, rhs: Self) -> Self {
		Self {
			x: self.x.min(rhs.x),
			y: self.y.min(rhs.y),
		}
	}

	/// per element max
	pub fn max(self, rhs: Self) -> Self {
		Self {
			x: self.x.max(rhs.x),
			y: self.y.max(rhs.y),
		}
	}

	pub fn min_component(self) -> T {
		self.x.min(self.y)
	}

	pub fn max_component(self) -> T {
		self.x.max(self.y)
	}

	pub fn min_dimension(self) -> usize {
		if self.x <= self.y { 0 } else { 1 }
	}

	pub fn max_dimension(self) -> usize {
		if self.x >= self.y { 0 } else { 1 }
	}

	pub fn permute(self, x: usize, y: usize) -> Self {
		Self {
			x: self[x],
			y: self[y],
		}
	}

	/// Horizontal product, return `self.x * self.y`.
	pub fn hprod(self) -> T {
		self.x * self.y
	}

	pub fn inside(self, b: &Bounds2<T>) -> bool {
		b.min.x <= self.x && self.x <= b.max.x && b.min.y <= self.y && self.y <= b.max.y
	}

	/// Variant of `inside` that excludes the upper boundary.
	pub fn inside_exclusive(self, b: &Bounds2<T>) -> bool {
		b.min.x <= self.x && self.x < b.max.x && b.min.y <= self.y && self.y < b.max.y
	}
}

impl Vector2f {
	pub fn is_normalized(self) -> bool {
		abs_diff_eq!(self.length(), 1.0)
	}

	pub fn length(self) -> Float {
		self.length_squared().sqrt()
	}

	/// Return the distance to `rhs`.
	pub fn distance(self, rhs: Self) -> Float {
		(self - rhs).length()
	}

	/// Return the squared distance to `rhs`.
	pub fn distance_squared(self, rhs: Self) -> Float {
		(self - rhs).length_squared()
	}

	pub fn normalize(&mut self) {
		let inv_len = 1.0 / self.length();
		self.x *= inv_len;
		self.y *= inv_len;
	}

	pub fn normalized(self) -> Self {
		let mut ret = self;
		ret.normalize();
		ret
	}

	pub fn ceil(self) -> Self {
		Self {
			x: self.x.ceil(),
			y: self.y.ceil(),
		}
	}

	pub fn floor(self) -> Self {
		Self {
			x: self.x.floor(),
			y: self.y.floor(),
		}
	}

	pub fn lerp(t: Float, a: Self, b: Self) -> Self {
		debug_assert!((0.0..=1.0).contains(&t));
		a + t * (b - a)
	}

	/// Fused multiply-add operation, return component-wise `a * b + c`.
	pub fn fma(a: Self, b: Self, c: Self) -> Self {
		Self {
			x: a.x.mul_add(b.x, c.x),
			y: a.y.mul_add(b.y, c.y),
		}
	}
}

impl<T: Display> Display for Vector2<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl<T: Default> Default for Vector2<T> {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}
}

impl<T> Index<usize> for Vector2<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.x,
			1 => &self.y,
			_ => panic!("out of bound"),
		}
	}
}

impl<T> IndexMut<usize> for Vector2<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			_ => panic!("out of bound"),
		}
	}
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl<T: Add<Output = T>> Add for Vector2<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T: AddAssign> AddAssign for Vector2<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T: SubAssign> SubAssign for Vector2<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl Mul<Vector2f> for Float {
	type Output = Vector2f;
	fn mul(self, rhs: Vector2f) -> Self::Output {
		rhs * self
	}
}

impl Mul<Vector2i> for i32 {
	type Output = Vector2i;
	fn mul(self, rhs: Vector2i) -> Self::Output {
		rhs * self
	}
}

impl Div<Float> for Vector2f {
	type Output = Self;
	fn div(self, rhs: Float) -> Self::Output {
		let inv = 1.0 / rhs;
		Self {
			x: self.x * inv,
			y: self.y * inv,
		}
	}
}

impl Div<i32> for Vector2i {
	type Output = Self;
	fn div(self, rhs: i32) -> Self::Output {
		let inv = 1.0 / rhs as Float;
		Self {
			x: (self.x as Float * inv) as i32,
			y: (self.y as Float * inv) as i32,
		}
	}
}

impl DivAssign<Float> for Vector2f {
	fn div_assign(&mut self, rhs: Float) {
		let inv = 1.0 / rhs;
		self.x *= inv;
		self.y *= inv;
	}
}

impl DivAssign<i32> for Vector2i {
	fn div_assign(&mut self, rhs: i32) {
		let inv = 1.0 / rhs as Float;
		self.x = (self.x as Float * inv) as i32;
		self.y = (self.y as Float * inv) as i32;
	}
}

impl AbsDiffEq for Vector2f {
	type Epsilon = Float;

	fn default_epsilon() -> Self::Epsilon {
		Float::EPSILON
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		self.x.abs_diff_eq(&other.x, epsilon) && self.y.abs_diff_eq(&other.y, epsilon)
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vector3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

pub type Vector3f = Vector3<Float>;
pub type Vector3i = Vector3<i32>;

impl<T: Number> Vector3<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		let ret = Self { x, y, z };
		debug_assert!(!ret.has_nan());
		ret
	}

	pub fn has_nan(self) -> bool {
		self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
	}

	pub fn abs(self) -> Self {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs(),
		}
	}

	pub fn length_squared(self) -> T {
		self.dot(self)
	}

	pub fn abs_dot(self, rhs: Self) -> T {
		self.dot(rhs).abs()
	}

	pub fn dot(self, rhs: Self) -> T {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	/// per element min
	pub fn min(self, rhs: Self) -> Self {
		Self {
			x: self.x.min(rhs.x),
			y: self.y.min(rhs.y),
			z: self.z.min(rhs.z),
		}
	}

	/// per element max
	pub fn max(self, rhs: Self) -> Self {
		Self {
			x: self.x.max(rhs.x),
			y: self.y.max(rhs.y),
			z: self.z.max(rhs.z),
		}
	}

	pub fn min_component(self) -> T {
		self.x.min(self.y).min(self.z)
	}

	pub fn max_component(self) -> T {
		self.x.max(self.y).max(self.z)
	}

	pub fn min_dimension(self) -> usize {
		if self.x <= self.y && self.x <= self.z {
			0
		} else if self.y <= self.z {
			1
		} else {
			2
		}
	}

	pub fn max_dimension(self) -> usize {
		if self.x >= self.y && self.x >= self.z {
			0
		} else if self.y >= self.z {
			1
		} else {
			2
		}
	}

	pub fn permute(self, x: usize, y: usize, z: usize) -> Self {
		Self {
			x: self[x],
			y: self[y],
			z: self[z],
		}
	}

	/// Horizontal product, return `self.x * self.y * self.z`.
	pub fn hprod(self) -> T {
		self.x * self.y * self.z
	}

	pub fn inside(self, b: &Bounds3<T>) -> bool {
		b.min.x <= self.x
			&& self.x <= b.max.x
			&& b.min.y <= self.y
			&& self.y <= b.max.y
			&& b.min.z <= self.z
			&& self.z <= b.max.z
	}

	/// Variant of `inside` that excludes the upper boundary.
	pub fn inside_exclusive(self, b: &Bounds3<T>) -> bool {
		b.min.x <= self.x
			&& self.x < b.max.x
			&& b.min.y <= self.y
			&& self.y < b.max.y
			&& b.min.z <= self.z
			&& self.z < b.max.z
	}
}

impl Vector3f {
	pub fn is_normalized(self) -> bool {
		abs_diff_eq!(self.length(), 1.0)
	}

	pub fn length(self) -> Float {
		self.length_squared().sqrt()
	}

	/// Return the distance to `rhs`.
	pub fn distance(self, rhs: Self) -> Float {
		(self - rhs).length()
	}

	/// Return the squared distance to `rhs`.
	pub fn distance_squared(self, rhs: Self) -> Float {
		(self - rhs).length_squared()
	}

	pub fn normalize(&mut self) {
		let inv_len = 1.0 / self.length();
		self.x *= inv_len;
		self.y *= inv_len;
		self.z *= inv_len;
	}

	pub fn normalized(self) -> Self {
		let mut ret = self;
		ret.normalize();
		ret
	}

	pub fn cross(self, rhs: Self) -> Self {
		// use f64 to increase precision
		let v1x = self.x as f64;
		let v1y = self.y as f64;
		let v1z = self.z as f64;
		let v2x = rhs.x as f64;
		let v2y = rhs.y as f64;
		let v2z = rhs.z as f64;

		Self {
			x: (v1y * v2z - v1z * v2y) as Float,
			y: (v1z * v2x - v1x * v2z) as Float,
			z: (v1x * v2y - v1y * v2x) as Float,
		}
	}

	pub fn ceil(self) -> Self {
		Self {
			x: self.x.ceil(),
			y: self.y.ceil(),
			z: self.z.ceil(),
		}
	}

	pub fn floor(self) -> Self {
		Self {
			x: self.x.floor(),
			y: self.y.floor(),
			z: self.z.floor(),
		}
	}

	pub fn lerp(t: Float, a: Self, b: Self) -> Self {
		debug_assert!((0.0..=1.0).contains(&t));
		a + t * (b - a)
	}

	/// Fused multiply-add operation, return component-wise `a * b + c`.
	pub fn fma(a: Self, b: Self, c: Self) -> Self {
		Self {
			x: a.x.mul_add(b.x, c.x),
			y: a.y.mul_add(b.y, c.y),
			z: a.z.mul_add(b.z, c.z),
		}
	}

	/// Construct a local coordinate system from a single normalized 3D vector.
	/// Return two orthonormal vectors that are perpendicular to `self`.
	pub fn coordinate_system(self) -> (Self, Self) {
		debug_assert!(self.is_normalized());

		let sign = (1.0 as Float).copysign(self.z);
		let a = -1.0 / (sign + self.z);
		let b = self.x * self.y * a;
		let v2 = Self {
			x: 1.0 + sign * self.x * self.x * a,
			y: sign * b,
			z: -sign * self.x,
		};
		let v3 = Self {
			x: b,
			y: sign + self.y * self.y * a,
			z: -self.y,
		};

		(v2, v3)
	}

	pub fn face_forward(self, v: Self) -> Self {
		if self.dot(v) < 0.0 { -self } else { self }
	}
}

impl<T: Display> Display for Vector3<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl<T: Default> Default for Vector3<T> {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
			z: T::default(),
		}
	}
}

impl<T> Index<usize> for Vector3<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("out of bound"),
		}
	}
}

impl<T> IndexMut<usize> for Vector3<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			_ => panic!("out of bound"),
		}
	}
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl<T: Add<Output = T>> Add for Vector3<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl<T: AddAssign> AddAssign for Vector3<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

impl<T: SubAssign> SubAssign for Vector3<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl Mul<Vector3f> for Float {
	type Output = Vector3f;
	fn mul(self, rhs: Vector3f) -> Self::Output {
		rhs * self
	}
}

impl Mul<Vector3i> for i32 {
	type Output = Vector3i;
	fn mul(self, rhs: Vector3i) -> Self::Output {
		rhs * self
	}
}

impl Div<Float> for Vector3f {
	type Output = Self;
	fn div(self, rhs: Float) -> Self::Output {
		let inv = 1.0 / rhs;
		Self {
			x: self.x * inv,
			y: self.y * inv,
			z: self.z * inv,
		}
	}
}

impl Div<i32> for Vector3i {
	type Output = Self;
	fn div(self, rhs: i32) -> Self::Output {
		let inv = 1.0 / rhs as Float;
		Self {
			x: (self.x as Float * inv) as i32,
			y: (self.y as Float * inv) as i32,
			z: (self.z as Float * inv) as i32,
		}
	}
}

impl DivAssign<Float> for Vector3f {
	fn div_assign(&mut self, rhs: Float) {
		let inv = 1.0 / rhs;
		self.x *= inv;
		self.y *= inv;
		self.z *= inv;
	}
}

impl DivAssign<i32> for Vector3i {
	fn div_assign(&mut self, rhs: i32) {
		let inv = 1.0 / rhs as Float;
		self.x = (self.x as Float * inv) as i32;
		self.y = (self.y as Float * inv) as i32;
		self.z = (self.z as Float * inv) as i32;
	}
}

impl AbsDiffEq for Vector3f {
	type Epsilon = Float;

	fn default_epsilon() -> Self::Epsilon {
		Float::EPSILON
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		self.x.abs_diff_eq(&other.x, epsilon)
			&& self.y.abs_diff_eq(&other.y, epsilon)
			&& self.z.abs_diff_eq(&other.z, epsilon)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bounds2<T> {
	pub min: Vector2<T>,
	pub max: Vector2<T>,
}

pub type Bounds2f = Bounds2<Float>;
pub type Bounds2i = Bounds2<i32>;

impl<T: Number> Bounds2<T> {
	pub fn new(p1: Vector2<T>, p2: Vector2<T>) -> Self {
		Self {
			min: p1.min(p2),
			max: p1.max(p2),
		}
	}

	/// Return a bounding box that encloses just a single point.
	pub fn from_point(p: Vector2<T>) -> Self {
		Self { min: p, max: p }
	}

	pub fn corner(&self, code: usize) -> Vector2<T> {
		Vector2 {
			x: self[code & 1].x,
			y: self[(code >> 1) & 1].y,
		}
	}

	pub fn union_point(&self, p: Vector2<T>) -> Self {
		Self {
			min: self.min.min(p),
			max: self.max.max(p),
		}
	}

	pub fn union(&self, rhs: &Self) -> Self {
		Self {
			min: self.min.min(rhs.min),
			max: self.max.max(rhs.max),
		}
	}

	pub fn intersect(&self, rhs: &Self) -> Self {
		Self {
			min: self.min.max(rhs.min),
			max: self.max.min(rhs.max),
		}
	}

	pub fn overlap(&self, rhs: &Self) -> bool {
		let x = self.min.x <= rhs.max.x && rhs.min.x <= self.max.x;
		let y = self.min.y <= rhs.max.y && rhs.min.y <= self.max.y;
		x && y
	}

	/// Return the squared distance to point `p`.
	pub fn distance_squared(&self, p: Vector2<T>) -> T {
		let dx = T::default().max(self.min.x - p.x).max(p.x - self.max.x);
		let dy = T::default().max(self.min.y - p.y).max(p.y - self.max.y);
		dx * dx + dy * dy
	}

	/// Return the distance to point `p`.
	pub fn distance(&self, p: Vector2<T>) -> Float {
		self.distance_squared(p).as_float().sqrt()
	}

	/// Return a new bounding box expanded by `delta` in all dimensions.
	pub fn expand(&self, delta: T) -> Self {
		let delta = Vector2::new(delta, delta);
		Self {
			min: self.min - delta,
			max: self.max + delta,
		}
	}

	pub fn diagonal(&self) -> Vector2<T> {
		self.max - self.min
	}

	pub fn area(&self) -> T {
		self.diagonal().hprod()
	}

	pub fn max_dimension(&self) -> usize {
		let d = self.diagonal();
		if d.x > d.y { 0 } else { 1 }
	}

	pub fn is_empty(&self) -> bool {
		self.min.x >= self.max.x || self.min.y >= self.max.y
	}

	pub fn is_degenerate(&self) -> bool {
		self.min.x > self.max.x || self.min.y > self.max.y
	}
}

impl Bounds2f {
	pub fn lerp(&self, t: Vector2f) -> Vector2f {
		Vector2f::new(
			lerp(t.x, self.min.x, self.max.x),
			lerp(t.y, self.min.y, self.max.y),
		)
	}

	/// Inverse of `lerp()`.
	pub fn offset(&self, p: Vector2f) -> Vector2f {
		let mut o = p - self.min;
		if self.min.x < self.max.x {
			o.x /= self.max.x - self.min.x;
		}
		if self.min.y < self.max.y {
			o.y /= self.max.y - self.min.y;
		}

		o
	}
}

impl<T: Number> Default for Bounds2<T> {
	/// Create an empty bounding box.
	fn default() -> Self {
		Self {
			min: Vector2 {
				x: T::MAX,
				y: T::MAX,
			},
			max: Vector2 {
				x: T::MIN,
				y: T::MIN,
			},
		}
	}
}

impl<T> Index<usize> for Bounds2<T> {
	type Output = Vector2<T>;

	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.min,
			1 => &self.max,
			_ => panic!("out of bound"),
		}
	}
}

impl<T> IndexMut<usize> for Bounds2<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.min,
			1 => &mut self.max,
			_ => panic!("out of bound"),
		}
	}
}

impl Bounds2i {
	pub fn iter(&self) -> Bounds2iIterator {
		Bounds2iIterator {
			bounds: self.clone(),
			p: self.min,
		}
	}
}

pub struct Bounds2iIterator {
	bounds: Bounds2i,
	p: Vector2i,
}

impl Iterator for Bounds2iIterator {
	type Item = Vector2i;

	fn next(&mut self) -> Option<Self::Item> {
		if self.p.y >= self.bounds.max.y {
			return None;
		}

		let ret = self.p;

		self.p.x += 1;
		if self.p.x == self.bounds.max.x {
			self.p.x = self.bounds.min.x;
			self.p.y += 1;
		}

		Some(ret)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bounds3<T> {
	pub min: Vector3<T>,
	pub max: Vector3<T>,
}

pub type Bounds3f = Bounds3<Float>;
pub type Bounds3i = Bounds3<i32>;

impl<T: Number> Bounds3<T> {
	pub fn new(p1: Vector3<T>, p2: Vector3<T>) -> Self {
		Self {
			min: p1.min(p2),
			max: p1.max(p2),
		}
	}

	/// Return a bounding box that encloses just a single point.
	pub fn from_point(p: Vector3<T>) -> Self {
		Self { min: p, max: p }
	}

	pub fn corner(&self, code: usize) -> Vector3<T> {
		Vector3 {
			x: self[code & 1].x,
			y: self[(code >> 1) & 1].y,
			z: self[(code >> 2) & 1].z,
		}
	}

	pub fn union_point(&self, p: Vector3<T>) -> Self {
		Self {
			min: self.min.min(p),
			max: self.max.max(p),
		}
	}

	pub fn union(&self, rhs: &Self) -> Self {
		Self {
			min: self.min.min(rhs.min),
			max: self.max.max(rhs.max),
		}
	}

	pub fn intersect(&self, rhs: &Self) -> Self {
		Self {
			min: self.min.max(rhs.min),
			max: self.max.min(rhs.max),
		}
	}

	pub fn overlap(&self, rhs: &Self) -> bool {
		let x = self.min.x <= rhs.max.x && rhs.min.x <= self.max.x;
		let y = self.min.y <= rhs.max.y && rhs.min.y <= self.max.y;
		let z = self.min.z <= rhs.max.z && rhs.min.z <= self.max.z;
		x && y && z
	}

	/// Return the squared distance to point `p`.
	pub fn distance_squared(&self, p: Vector3<T>) -> T {
		let dx = T::default().max(self.min.x - p.x).max(p.x - self.max.x);
		let dy = T::default().max(self.min.y - p.y).max(p.y - self.max.y);
		let dz = T::default().max(self.min.z - p.z).max(p.z - self.max.z);
		dx * dx + dy * dy + dz * dz
	}

	/// Return the distance to point `p`.
	pub fn distance(&self, p: Vector3<T>) -> Float {
		self.distance_squared(p).as_float().sqrt()
	}

	/// Return a new bounding box expanded by `delta` in all dimensions.
	pub fn expand(&self, delta: T) -> Self {
		let delta = Vector3::new(delta, delta, delta);
		Self {
			min: self.min - delta,
			max: self.max + delta,
		}
	}

	pub fn diagonal(&self) -> Vector3<T> {
		self.max - self.min
	}

	pub fn surface_area(&self) -> T {
		let d = self.diagonal();
		let area = d.x * d.y + d.x * d.z + d.y * d.z;
		area + area
	}

	pub fn volume(&self) -> T {
		self.diagonal().hprod()
	}

	pub fn max_dimension(&self) -> usize {
		let d = self.diagonal();
		if d.x > d.y && d.x > d.z {
			0
		} else if d.y > d.z {
			1
		} else {
			2
		}
	}

	pub fn is_empty(&self) -> bool {
		self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
	}

	pub fn is_degenerate(&self) -> bool {
		self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
	}
}

impl Bounds3f {
	pub fn lerp(&self, t: Vector3f) -> Vector3f {
		Vector3f::new(
			lerp(t.x, self.min.x, self.max.x),
			lerp(t.y, self.min.y, self.max.y),
			lerp(t.z, self.min.z, self.max.z),
		)
	}

	/// Inverse of `lerp()`.
	pub fn offset(&self, p: Vector3f) -> Vector3f {
		let mut o = p - self.min;
		if self.min.x < self.max.x {
			o.x /= self.max.x - self.min.x;
		}
		if self.min.y < self.max.y {
			o.y /= self.max.y - self.min.y;
		}
		if self.min.z < self.max.z {
			o.z /= self.max.z - self.min.z;
		}

		o
	}

	pub fn bounding_sphere(&self) -> (Vector3f, Float) {
		let center = (self.min + self.max) / 2.0;
		let radius = if center.inside(self) {
			center.distance(self.max)
		} else {
			0.0
		};

		(center, radius)
	}
}

impl<T: Number> Default for Bounds3<T> {
	/// Create an empty bounding box.
	fn default() -> Self {
		Self {
			min: Vector3 {
				x: T::MAX,
				y: T::MAX,
				z: T::MAX,
			},
			max: Vector3 {
				x: T::MIN,
				y: T::MIN,
				z: T::MIN,
			},
		}
	}
}

impl<T> Index<usize> for Bounds3<T> {
	type Output = Vector3<T>;

	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.min,
			1 => &self.max,
			_ => panic!("out of bound"),
		}
	}
}

impl<T> IndexMut<usize> for Bounds3<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.min,
			1 => &mut self.max,
			_ => panic!("out of bound"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;

	#[test]
	fn test_indexing() {
		let v = Vector2i::new(1, 2);
		assert_eq!(v[0], 1);
		assert_eq!(v[1], 2);

		let v = Vector3i::new(1, 2, 3);
		assert_eq!(v[0], 1);
		assert_eq!(v[1], 2);
		assert_eq!(v[2], 3);
	}

	#[test]
	#[should_panic]
	fn test_out_of_bounds_v2() {
		let v = Vector2i::default();
		v[2];
	}

	#[test]
	#[should_panic]
	fn test_out_of_bounds_v3() {
		let v = Vector3i::default();
		v[3];
	}

	#[test]
	fn test_dot() {
		let a = Vector2f::new(1.0, -2.0);
		let b = Vector2f::new(3.0, 4.0);
		assert_eq!(a.dot(b), -5.0);
		assert_eq!(a.abs_dot(b), 5.0);

		let a = Vector3f::new(1.0, 2.0, -3.0);
		let b = Vector3f::new(4.0, 5.0, 6.0);
		assert_eq!(a.dot(b), -4.0);
		assert_eq!(a.abs_dot(b), 4.0);
	}

	#[test]
	fn test_normalize() {
		let v = Vector2f::new(3.0, 4.0).normalized();
		assert_eq!(v.length(), 1.0);

		let v = Vector3f::new(0.0, 3.0, 4.0).normalized();
		assert_eq!(v.length(), 1.0);
	}

	#[test]
	fn test_cross() {
		let a = Vector3f::new(1.0, 0.0, 0.0);
		let b = Vector3f::new(0.0, 1.0, 0.0);
		assert_eq!(a.cross(b), Vector3f::new(0.0, 0.0, 1.0));
		assert_eq!(b.cross(a), Vector3f::new(0.0, 0.0, -1.0));
	}

	#[test]
	fn test_lerp() {
		let a = Vector2f::new(1.0, 2.0);
		let b = Vector2f::new(4.0, 5.0);
		assert_eq!(Vector2f::lerp(0.25, a, b), Vector2f::new(1.75, 2.75));

		let a = Vector3f::new(1.0, 2.0, 3.0);
		let b = Vector3f::new(4.0, 5.0, 6.0);
		assert_eq!(Vector3f::lerp(0.25, a, b), Vector3f::new(1.75, 2.75, 3.75));
	}

	#[test]
	fn test_coordinate_system() {
		let v1 = Vector3f::new(1.0, 2.0, 3.0).normalized();
		let (v2, v3) = v1.coordinate_system();
		let v1xv2 = v1.cross(v2);

		assert_abs_diff_eq!(v2.length(), 1.0);
		assert_abs_diff_eq!(v3.length(), 1.0);
		assert_abs_diff_eq!(v1xv2, v3);
	}

	#[test]
	fn test_new_bounds() {
		let p1 = Vector2i::new(1, -1);
		let p2 = Vector2i::new(-1, 1);
		let bounds = Bounds2i::new(p1, p2);
		assert_eq!(bounds.min, Vector2i::new(-1, -1));
		assert_eq!(bounds.max, Vector2i::new(1, 1));

		let p1 = Vector3i::new(1, -1, 1);
		let p2 = Vector3i::new(-1, 1, -1);
		let bounds = Bounds3i::new(p1, p2);
		assert_eq!(bounds.min, Vector3i::new(-1, -1, -1));
		assert_eq!(bounds.max, Vector3i::new(1, 1, 1));
	}

	#[test]
	fn test_corner() {
		let p1 = Vector2i::new(1, 2);
		let p2 = Vector2i::new(3, 4);
		let bounds = Bounds2i::new(p1, p2);
		assert_eq!(bounds.corner(0), Vector2i::new(1, 2));
		assert_eq!(bounds.corner(1), Vector2i::new(3, 2));
		assert_eq!(bounds.corner(2), Vector2i::new(1, 4));
		assert_eq!(bounds.corner(3), Vector2i::new(3, 4));

		let p1 = Vector3i::new(1, 2, 3);
		let p2 = Vector3i::new(4, 5, 6);
		let bounds = Bounds3i::new(p1, p2);
		assert_eq!(bounds.corner(0), Vector3i::new(1, 2, 3));
		assert_eq!(bounds.corner(1), Vector3i::new(4, 2, 3));
		assert_eq!(bounds.corner(2), Vector3i::new(1, 5, 3));
		assert_eq!(bounds.corner(3), Vector3i::new(4, 5, 3));
		assert_eq!(bounds.corner(4), Vector3i::new(1, 2, 6));
		assert_eq!(bounds.corner(5), Vector3i::new(4, 2, 6));
		assert_eq!(bounds.corner(6), Vector3i::new(1, 5, 6));
		assert_eq!(bounds.corner(7), Vector3i::new(4, 5, 6));
	}

	#[test]
	fn test_union() {
		let empty = Bounds2i::default();
		let bounds = Bounds2i::new(Vector2i::new(1, 2), Vector2i::new(3, 4));
		assert_eq!(empty.union(&empty), empty);
		assert_eq!(bounds.union(&empty), bounds);

		let b1 = Bounds2i::new(Vector2i::new(-1, -1), Vector2i::new(1, 2));
		let b2 = Bounds2i::new(Vector2i::new(0, 0), Vector2i::new(2, 3));
		assert_eq!(
			b1.union(&b2),
			Bounds2i::new(Vector2i::new(-1, -1), Vector2i::new(2, 3))
		);

		let empty = Bounds3i::default();
		let bounds = Bounds3i::new(Vector3i::new(1, 2, 3), Vector3i::new(4, 5, 6));
		assert_eq!(empty.union(&empty), empty);
		assert_eq!(bounds.union(&empty), bounds);

		let b1 = Bounds3i::new(Vector3i::new(-1, -1, -1), Vector3i::new(1, 2, 3));
		let b2 = Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(3, 2, 1));
		assert_eq!(
			b1.union(&b2),
			Bounds3i::new(Vector3i::new(-1, -1, -1), Vector3i::new(3, 2, 3))
		);
	}

	#[test]
	fn test_intersect() {
		let b1 = Bounds2i::new(Vector2i::new(-1, -2), Vector2i::new(1, 2));
		let b2 = Bounds2i::new(Vector2i::new(-2, -1), Vector2i::new(2, 1));
		assert_eq!(
			b1.intersect(&b2),
			Bounds2i::new(Vector2i::new(-1, -1), Vector2i::new(1, 1))
		);

		let b1 = Bounds3i::new(Vector3i::new(-1, -1, -1), Vector3i::new(1, 2, 3));
		let b2 = Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(3, 2, 1));
		assert_eq!(
			b1.intersect(&b2),
			Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(1, 2, 1))
		);
	}

	#[test]
	fn test_overlap() {
		let b1 = Bounds2i::new(Vector2i::new(0, 0), Vector2i::new(1, 1));
		let b2 = Bounds2i::new(Vector2i::new(-1, -1), Vector2i::new(0, 0));
		assert!(b1.overlap(&b2));

		let b1 = Bounds2i::new(Vector2i::new(1, 2), Vector2i::new(3, 4));
		let b2 = Bounds2i::new(Vector2i::new(0, 0), Vector2i::new(1, 1));
		assert!(!b1.overlap(&b2));

		let b1 = Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(1, 1, 1));
		let b2 = Bounds3i::new(Vector3i::new(-1, -1, -1), Vector3i::new(0, 0, 0));
		assert!(b1.overlap(&b2));

		let b1 = Bounds3i::new(Vector3i::new(1, 2, 3), Vector3i::new(4, 5, 6));
		let b2 = Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(1, 1, 1));
		assert!(!b1.overlap(&b2));
	}

	#[test]
	fn test_distance_between_bounds_and_point() {
		let b = Bounds2i::new(Vector2i::new(0, 0), Vector2i::new(1, 1));
		let p = Vector2i::new(0, 2);
		assert_eq!(b.distance(p), 1.0);

		let b = Bounds2i::new(Vector2i::new(0, 0), Vector2i::new(1, 1));
		let p = Vector2i::new(4, 5);
		assert_eq!(b.distance(p), 5.0);

		let b = Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(1, 1, 1));
		let p = Vector3i::new(0, 0, 2);
		assert_eq!(b.distance(p), 1.0);

		let b = Bounds3i::new(Vector3i::new(0, 0, 0), Vector3i::new(1, 1, 1));
		let p = Vector3i::new(0, 4, 5);
		assert_eq!(b.distance(p), 5.0);
	}

	#[test]
	fn test_area_and_volume() {
		let b = Bounds2f::new(Vector2f::new(1.0, 2.0), Vector2f::new(3.0, 4.0));
		assert_eq!(b.area(), 4.0);

		let b = Bounds3f::new(Vector3f::new(1.0, 2.0, 3.0), Vector3f::new(4.0, 5.0, 6.0));
		assert_eq!(b.volume(), 27.0);
		assert_eq!(b.surface_area(), 54.0);
	}

	#[test]
	fn test_bounds_lerp_and_offset() {
		let b = Bounds2f::new(Vector2f::new(1.0, 2.0), Vector2f::new(3.0, 4.0));
		let t = Vector2f::new(0.1, 0.2);
		let p = Vector2f::new(1.2, 2.4);
		assert_eq!(b.lerp(t), p);
		assert_abs_diff_eq!(b.offset(p), t);

		let b = Bounds3f::new(Vector3f::new(1.0, 2.0, 3.0), Vector3f::new(4.0, 5.0, 6.0));
		let t = Vector3f::new(0.1, 0.2, 0.3);
		let p = Vector3f::new(1.3, 2.6, 3.9);
		assert_eq!(b.lerp(t), p);
		assert_abs_diff_eq!(b.offset(p), t);
	}

	#[test]
	fn test_bounds2i_iterator() {
		let b = Bounds2i::default();
		assert!(b.iter().collect::<Vec<_>>().is_empty());

		let empty = Bounds2i::new(Vector2i::new(0, 0), Vector2i::new(0, 0));
		assert!(empty.is_empty());
		assert!(!empty.is_degenerate());
		assert!(empty.iter().collect::<Vec<_>>().is_empty());

		let b = Bounds2i::new(Vector2i::new(-1, -1), Vector2i::new(1, 2));
		let array = vec![
			Vector2i::new(-1, -1),
			Vector2i::new(0, -1),
			Vector2i::new(-1, 0),
			Vector2i::new(0, 0),
			Vector2i::new(-1, 1),
			Vector2i::new(0, 1),
		];
		assert_eq!(b.iter().collect::<Vec<_>>(), array);
	}
}
