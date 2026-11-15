use super::{number::HasNaN, vecmath::Vec3f};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Ray {
	/// Origin of the ray
	pub o: Vec3f,
	/// Direction of the ray
	pub d: Vec3f,
	// this is for animated objects, default is 0
	pub time: f32,
}

impl Ray {
	pub fn new(o: Vec3f, d: Vec3f) -> Self {
		Self::new_with_time(o, d, 0.)
	}

	pub fn new_with_time(o: Vec3f, d: Vec3f, time: f32) -> Self {
		Self { o, d, time }
	}

	pub fn eval(&self, t: f32) -> Vec3f {
		self.o + self.d * t
	}
}

impl HasNaN for Ray {
	fn has_nan(&self) -> bool {
		self.o.has_nan() || self.d.has_nan() || self.time.is_nan()
	}
}

#[derive(Debug, Clone)]
pub struct RayDifferential {
	pub ray: Ray,
	pub has_differentials: bool,
	/// Following members are only valid if `has_differentials` is true.
	pub rx_origin: Vec3f,
	pub ry_origin: Vec3f,
	pub rx_direction: Vec3f,
	pub ry_direction: Vec3f,
}

impl RayDifferential {
	pub fn new(o: Vec3f, d: Vec3f, time: f32) -> Self {
		Ray::new_with_time(o, d, time).into()
	}

	pub fn scale_differentials(&mut self, s: f32) {
		self.rx_origin *= s;
		self.rx_direction *= s;
		self.ry_origin *= s;
		self.ry_direction *= s;
	}
}

impl From<Ray> for RayDifferential {
	fn from(ray: Ray) -> Self {
		Self {
			ray,
			has_differentials: false,
			rx_origin: Vec3f::default(),
			ry_origin: Vec3f::default(),
			rx_direction: Vec3f::default(),
			ry_direction: Vec3f::default(),
		}
	}
}

impl From<&Ray> for RayDifferential {
	fn from(ray: &Ray) -> Self {
		ray.clone().into()
	}
}

impl HasNaN for RayDifferential {
	fn has_nan(&self) -> bool {
		self.ray.has_nan()
			|| (self.has_differentials
				&& (self.rx_origin.has_nan()
					|| self.ry_origin.has_nan()
					|| self.rx_direction.has_nan()
					|| self.ry_direction.has_nan()))
	}
}

impl Deref for RayDifferential {
	type Target = Ray;
	fn deref(&self) -> &Self::Target {
		&self.ray
	}
}

impl DerefMut for RayDifferential {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.ray
	}
}
