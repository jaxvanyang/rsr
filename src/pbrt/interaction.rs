use super::{
	ray::{Ray, RayDifferential},
	vecmath::{Vec2f, Vec3f},
};

#[derive(Debug, Default)]
pub struct Interaction {
	/// The interaction point.
	pub p: Vec3f,
	/// time
	pub t: f32,
	/// `omega_o`, the negative ray direction.
	pub wo: Vec3f,
	/// The surface normal.
	pub n: Vec3f,
	/// (u, v) parametric coordinates.
	pub uv: Vec2f,
	// TODO: medium_interface, medium
}

impl Interaction {
	pub fn new(p: Vec3f, n: Vec3f, uv: Vec2f, wo: Vec3f, t: f32) -> Self {
		Self { p, t, wo, n, uv }
	}

	pub fn is_surface_interaction(&self) -> bool {
		!self.is_medium_interaction()
	}

	pub fn is_medium_interaction(&self) -> bool {
		self.n == Vec3f::default()
	}

	pub fn offset_ray_origin(&self, d: Vec3f) -> Vec3f {
		Ray::offset_ray_origin(self.p, self.n, d)
	}

	/// `p` is the ray's destination
	pub fn offset_ray_origin_p(&self, p: Vec3f) -> Vec3f {
		Ray::offset_ray_origin(self.p, self.n, p - self.p)
	}

	pub fn spawn_ray(&self, d: Vec3f) -> RayDifferential {
		RayDifferential::new(self.offset_ray_origin(d), d, self.t)
	}

	pub fn spawn_ray_to(&self, p: Vec3f) -> Ray {
		Ray::spawn_ray_to(self.p, self.n, self.t, p)
	}
}

#[derive(Debug)]
pub struct Shading {
	pub n: Vec3f,
	pub dpdu: Vec3f,
	pub dpdv: Vec3f,
	pub dndu: Vec3f,
	pub dndv: Vec3f,
}

#[derive(Debug)]
pub struct SurfaceInteraction {
	pub base: Interaction,
	pub dpdu: Vec3f,
	pub dpdv: Vec3f,
	pub dndu: Vec3f,
	pub dndv: Vec3f,
	pub shading: Shading,
	pub face_index: usize,
	// TODO: material, area_light
	pub dpdx: Vec3f,
	pub dpdy: Vec3f,
	pub dudx: f32,
	pub dvdx: f32,
	pub dudy: f32,
	pub dvdy: f32,
}

impl SurfaceInteraction {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		p: Vec3f,
		uv: Vec2f,
		wo: Vec3f,
		dpdu: Vec3f,
		dpdv: Vec3f,
		dndu: Vec3f,
		dndv: Vec3f,
		time: f32,
		flip_normal: bool,
	) -> Self {
		let mut base = Interaction::new(p, dpdu.cross(dpdv).normalized(), uv, wo, time);
		// initialize shading geometry from true geometry
		let mut shading = Shading { n: base.n, dpdu, dpdv, dndu, dndv };

		// adujust normal based on orientation and handedness
		if flip_normal {
			base.n = -base.n;
			shading.n = -shading.n;
		}

		Self {
			base,
			dpdu,
			dpdv,
			dndu,
			dndv,
			shading,
			face_index: 0,
			dpdx: Vec3f::default(),
			dpdy: Vec3f::default(),
			dudx: 0.0,
			dvdx: 0.0,
			dudy: 0.0,
			dvdy: 0.0,
		}
	}

	pub fn set_shading_geometry(
		&mut self,
		n: Vec3f,
		dpdu: Vec3f,
		dpdv: Vec3f,
		dndu: Vec3f,
		dndv: Vec3f,
		orientation_is_authoritative: bool,
	) {
		self.shading.n = n;
		self.shading.dpdu = dpdu;
		self.shading.dpdv = dpdv;
		self.shading.dndu = dndu;
		self.shading.dndv = dndv;

		if orientation_is_authoritative {
			self.base.n = self.base.n.face_forward(n);
		} else {
			self.shading.n = n.face_forward(self.base.n);
		}
	}
}

pub struct MediumInteraction {
	pub base: Interaction,
	// TODO: pub phase: PhaseFunction,
}

// TODO: impl MediumInteraction
