use super::{
	Float,
	vecmath::{Vector2f, Vector3f, Vector3fi},
};

pub struct Interaction {
	/// The interaction point using interval arithmetic.
	pub pi: Vector3fi,
	pub time: Float,
	/// `omega_o`, the negative ray direction.
	pub wo: Vector3f,
	/// The surface normal.
	pub n: Vector3f,
	/// (u, v) parametric coordinates.
	pub uv: Vector2f,
	// TODO: medium_interface, medium
}

impl Interaction {
	pub fn new(pi: Vector3fi, n: Vector3f, uv: Vector2f, wo: Vector3f, time: Float) -> Self {
		Self { pi, time, wo, n, uv }
	}

	/// Return the interaction point without error.
	pub fn p(&self) -> Vector3f {
		self.pi.into()
	}

	pub fn is_surface_interaction(&self) -> bool {
		!self.is_medium_interaction()
	}

	pub fn is_medium_interaction(&self) -> bool {
		self.n == Vector3f::default()
	}
}

pub struct Shading {
	pub n: Vector3f,
	pub dpdu: Vector3f,
	pub dpdv: Vector3f,
	pub dndu: Vector3f,
	pub dndv: Vector3f,
}

pub struct SurfaceInteraction {
	pub interaction: Interaction,
	pub dpdu: Vector3f,
	pub dpdv: Vector3f,
	pub dndu: Vector3f,
	pub dndv: Vector3f,
	pub shading: Shading,
	pub face_index: usize,
	// TODO: material, area_light
	pub dpdx: Vector3f,
	pub dpdy: Vector3f,
	pub dudx: Float,
	pub dvdx: Float,
	pub dudy: Float,
	pub dvdy: Float,
}

impl SurfaceInteraction {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		pi: Vector3fi,
		uv: Vector2f,
		wo: Vector3f,
		dpdu: Vector3f,
		dpdv: Vector3f,
		dndu: Vector3f,
		dndv: Vector3f,
		time: Float,
		flip_normal: bool,
	) -> Self {
		let mut interaction = Interaction::new(pi, dpdu.cross(dpdv).normalized(), uv, wo, time);
		// initialize shading geometry from true geometry
		let mut shading = Shading { n: interaction.n, dpdu, dpdv, dndu, dndv };

		// adujust normal based on orientation and handedness
		if flip_normal {
			interaction.n = -interaction.n;
			shading.n = -shading.n;
		}

		Self {
			interaction,
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
			self.interaction.n = self.interaction.n.face_forward(n);
		} else {
			self.shading.n = n.face_forward(self.interaction.n);
		}
	}
}

pub struct MediumInteraction {
	pub interaction: Interaction,
	// TODO: pub phase: PhaseFunction,
}

// TODO: impl MediumInteraction
