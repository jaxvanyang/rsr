use super::Float;
use number::Number;
use std::ops::*;

mod number {
	pub trait Number {
		fn abs(&self) -> Self;
	}

	impl Number for f32 {
		fn abs(&self) -> Self {
			f32::abs(*self)
		}
	}

	impl Number for i32 {
		fn abs(&self) -> Self {
			i32::abs(*self)
		}
	}

	impl Number for f64 {
		fn abs(&self) -> Self {
			f64::abs(*self)
		}
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vector3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

pub type Vector2f = Vector2<Float>;
pub type Vector2i = Vector2<i32>;
pub type Vector3f = Vector3<Float>;
pub type Vector3i = Vector3<i32>;

impl<T> Vector2<T> {
	pub fn abs(&self) -> Self
	where
		T: Number,
	{
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
		}
	}

	pub fn dot(&self, rhs: &Self) -> T
	where
		T: Mul<Output = T> + Add<Output = T> + Copy,
	{
		self.x * rhs.x + self.y * rhs.y
	}

	pub fn abs_dot(&self, rhs: &Self) -> T
	where
		T: Mul<Output = T> + Add<Output = T> + Copy + Number,
	{
		self.dot(rhs).abs()
	}
}

impl Vector2f {
	pub fn has_nan(&self) -> bool {
		self.x.is_nan() || self.y.is_nan()
	}

	pub fn new(x: Float, y: Float) -> Self {
		let ret = Self { x, y };
		debug_assert!(!ret.has_nan());
		ret
	}
}

impl Vector2i {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
}

impl<T> Vector3<T> {
	pub fn abs(&self) -> Self
	where
		T: Number,
	{
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs(),
		}
	}

	pub fn dot(&self, rhs: &Self) -> T
	where
		T: Mul<Output = T> + Add<Output = T> + Copy,
	{
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn abs_dot(&self, rhs: &Self) -> T
	where
		T: Mul<Output = T> + Add<Output = T> + Copy + Number,
	{
		self.dot(rhs).abs()
	}

	pub fn length_squared(&self) -> T
	where
		T: Mul<Output = T> + Add<Output = T> + Copy,
	{
		self.dot(self)
	}

	pub fn min_component(&self) -> T
	where
		T: Ord + Copy,
	{
		self.x.min(self.y).min(self.z)
	}

	pub fn max_component(&self) -> T
	where
		T: Ord + Copy,
	{
		self.x.max(self.y).max(self.z)
	}

	pub fn max_dimension(&self) -> usize
	where
		T: Ord,
	{
		if self.x > self.y && self.x > self.z {
			0
		} else if self.y > self.z {
			1
		} else {
			2
		}
	}

	/// per element min
	pub fn min(&self, rhs: &Self) -> Self
	where
		T: Ord + Copy,
	{
		Self {
			x: self.x.min(rhs.x),
			y: self.y.min(rhs.y),
			z: self.z.min(rhs.z),
		}
	}

	/// per element max
	pub fn max(&self, rhs: &Self) -> Self
	where
		T: Ord + Copy,
	{
		Self {
			x: self.x.max(rhs.x),
			y: self.y.max(rhs.y),
			z: self.z.max(rhs.z),
		}
	}

	pub fn permute(&self, x: usize, y: usize, z: usize) -> Self
	where
		T: Copy,
	{
		Self {
			x: self[x],
			y: self[y],
			z: self[z],
		}
	}
}

impl Vector3f {
	pub fn has_nan(&self) -> bool {
		self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
	}

	pub fn new(x: Float, y: Float, z: Float) -> Self {
		let ret = Self { x, y, z };
		debug_assert!(!ret.has_nan());
		ret
	}

	pub fn cross(&self, rhs: &Self) -> Self {
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

	pub fn length(&self) -> Float {
		self.length_squared().sqrt()
	}

	pub fn normalize(&self) -> Self {
		*self / self.length_squared()
	}
}

impl Vector3i {
	pub fn new(x: i32, y: i32, z: i32) -> Self {
		Self { x, y, z }
	}

	pub fn length(&self) -> i32 {
		(self.length_squared() as Float).sqrt() as i32
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

impl<T: Default> Default for Vector2<T> {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
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

impl<T: Neg<Output = T>> Neg for Vector2<T> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
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

impl<T: Default> Default for Vector3<T> {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
			z: T::default(),
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

#[cfg(test)]
mod tests {
	use super::*;

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
}
