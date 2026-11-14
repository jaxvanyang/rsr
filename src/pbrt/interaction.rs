use super::vecmath::{Vector2f, Vector3f};

#[derive(Debug)]
pub struct Interaction {
	/// The interaction point.
	pub p: Vector3f,
	/// time
	pub t: f32,
	/// `omega_o`, the negative ray direction.
	pub wo: Vector3f,
	/// The surface normal.
	pub n: Vector3f,
	/// (u, v) parametric coordinates.
	pub uv: Vector2f,
	// TODO: medium_interface, medium
}

impl Interaction {
	pub fn new(p: Vector3f, n: Vector3f, uv: Vector2f, wo: Vector3f, t: f32) -> Self {
		Self { p, t, wo, n, uv }
	}

	pub fn is_surface_interaction(&self) -> bool {
		!self.is_medium_interaction()
	}

	pub fn is_medium_interaction(&self) -> bool {
		self.n == Vector3f::default()
	}
}

#[derive(Debug)]
pub struct Shading {
	pub n: Vector3f,
	pub dpdu: Vector3f,
	pub dpdv: Vector3f,
	pub dndu: Vector3f,
	pub dndv: Vector3f,
}

#[derive(Debug)]
pub struct SurfaceInteraction {
	pub base: Interaction,
	pub dpdu: Vector3f,
	pub dpdv: Vector3f,
	pub dndu: Vector3f,
	pub dndv: Vector3f,
	pub shading: Shading,
	pub face_index: usize,
	// TODO: material, area_light
	pub dpdx: Vector3f,
	pub dpdy: Vector3f,
	pub dudx: f32,
	pub dvdx: f32,
	pub dudy: f32,
	pub dvdy: f32,
}

impl SurfaceInteraction {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		p: Vector3f,
		uv: Vector2f,
		wo: Vector3f,
		dpdu: Vector3f,
		dpdv: Vector3f,
		dndu: Vector3f,
		dndv: Vector3f,
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
			dpdx: Vector3f::default(),
			dpdy: Vector3f::default(),
			dudx: 0.0,
			dvdx: 0.0,
			dudy: 0.0,
			dvdy: 0.0,
		}
	}

	pub fn set_shading_geometry(
		&mut self,
		n: Vector3f,
		dpdu: Vector3f,
		dpdv: Vector3f,
		dndu: Vector3f,
		dndv: Vector3f,
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
