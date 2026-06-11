use super::{Float, number::HasNaN, vecmath::Vector3f};

#[derive(Debug, Clone)]
pub struct Ray {
	/// Origin of the ray
	pub o: Vector3f,
	/// Direction of the ray
	pub d: Vector3f,
	pub time: Float,
	// TODO: medium
}

impl Ray {
	pub fn new(o: Vector3f, d: Vector3f, time: Float) -> Self {
		Self { o, d, time }
	}

	pub fn eval(&self, t: Float) -> Vector3f {
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
	pub rx_origin: Vector3f,
	pub ry_origin: Vector3f,
	pub rx_direction: Vector3f,
	pub ry_direction: Vector3f,
}

impl RayDifferential {
	pub fn new(o: Vector3f, d: Vector3f, time: Float) -> Self {
		Ray::new(o, d, time).into()
	}

	pub fn scale_differentials(&mut self, s: Float) {
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
			rx_origin: Vector3f::default(),
			ry_origin: Vector3f::default(),
			rx_direction: Vector3f::default(),
			ry_direction: Vector3f::default(),
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
