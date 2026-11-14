use std::{f32::consts::PI, fmt::Debug};

use super::{
	Bounds3f, Transform, Vector2f, Vector3f,
	interaction::{Interaction, MediumInteraction, SurfaceInteraction},
	math::{diff_of_products, safe_acos, safe_sqrt},
	ray::Ray,
};

pub trait Shape: Debug {
	fn bounds(&self) -> Bounds3f;
	// fn normal_bounds(&self) -> DirectionCone;
	fn intersect(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection>;
	fn intersect_p(&self, ray: &Ray, tmax: f32) -> bool;
	fn area(&self) -> f32;
	fn sample(&self, u: Vector2f) -> Option<ShapeSample>;
	fn pdf(&self, intr: &Interaction) -> f32;
	fn sample_ctx(&self, ctx: &ShapeSampleContext, u: Vector2f) -> Option<ShapeSample>;
	/// `wi` is the incident direction
	fn pdf_ctx(&self, ctx: &ShapeSampleContext, wi: Vector3f) -> f32;
}

#[derive(Debug)]
pub struct ShapeSampleContext {
	/// reference point
	pub p: Vector3f,
	/// normal
	pub n: Vector3f,
	/// shading normal
	pub ns: Vector3f,
	/// time
	pub t: f32,
}

impl ShapeSampleContext {
	pub fn new(p: Vector3f, n: Vector3f, ns: Vector3f, t: f32) -> Self {
		Self { p, n, ns, t }
	}

	pub fn from_surface_interaction(intr: &SurfaceInteraction) -> Self {
		Self { p: intr.base.p, n: intr.base.n, ns: intr.shading.n, t: intr.base.t }
	}

	pub fn from_medium_interaction(intr: &MediumInteraction) -> Self {
		Self { p: intr.base.p, n: Vector3f::default(), ns: Vector3f::default(), t: intr.base.t }
	}

	// TODO: OffsetRayOrigin(), SpawnRay()
}

#[derive(Debug)]
pub struct ShapeSample {
	/// interaction
	pub intr: Interaction,
	pub pdf: f32,
}

#[derive(Debug)]
pub struct ShapeIntersection {
	/// interaction
	pub intr: SurfaceInteraction,
	/// time of hit
	pub t: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sphere {
	/// radius
	r: f32,
	z_min: f32,
	z_max: f32,
	/// theta value of `z_min`
	theta_z_min: f32,
	/// theta value of `z_max`
	theta_z_max: f32,
	phi_max: f32,
	render_from_object: Transform,
	/// whether the normal is reversed
	is_reversed: bool,
	/// whether the transform swaps handedness
	is_handedness_swapped: bool,
}

impl Sphere {
	pub fn new(
		render_from_object: &Transform,
		is_reversed: bool,
		radius: f32,
		z_min: f32,
		z_max: f32,
		phi_max: f32,
	) -> Self {
		debug_assert!(z_min <= z_max);
		let z_min = z_min.clamp(-radius, radius);
		let z_max = z_max.clamp(-radius, radius);
		let theta_z_min = (z_min / radius).acos();
		let theta_z_max = (z_max / radius).acos();
		Self {
			r: radius,
			z_min,
			z_max,
			theta_z_min,
			theta_z_max,
			phi_max,
			render_from_object: render_from_object.clone(),
			is_reversed,
			is_handedness_swapped: render_from_object.swaps_handedness(),
		}
	}

	pub fn basic_intersect(&self, ray: &Ray, tmax: f32) -> Option<QuadricIntersection> {
		// transform ray origin and direction to object space
		let o = self.render_from_object.invert_point(ray.o);
		let d = self.render_from_object.invert_vector(ray.d);
		// solve quadratic equation to compute sphere t0 and t1
		// compute sphere quadratic coefficients
		let a = d.x.powi(2) + d.y.powi(2) + d.z.powi(2);
		let b = 2.0 * (d.x * o.x + d.y * o.y + d.z * o.z);
		let c = o.x.powi(2) + o.y.powi(2) + o.z.powi(2) - self.r.powi(2);
		// compute sphere quadratic discriminant discrim
		let discrim = b * b - 4.0 * a * c;
		let root_discrim = discrim.sqrt();
		// compute quadratic t values
		let q = -0.5 * (b + root_discrim);
		let t0 = q / a;
		let t1 = c / q;
		let mut t = t0;
		debug_assert!(t0 <= t1);
		// check quadric shape t0 and t1 for nearest intersection
		if t0 > tmax || t1 <= 0.0 {
			return None;
		}
		// compute sphere hit position and phi
		let mut p = o + t * d;
		if p.x == 0.0 && p.y == 0.0 {
			p.x = 1e-5 * self.r;
		}
		let mut phi = p.y.atan2(p.x);
		if phi < 0.0 {
			phi += 2.0 * PI;
		}
		// test sphere intersection against clipping parameters
		if (self.z_min > -self.r && p.z < self.z_min)
			|| (self.z_max < self.r && p.z > self.z_max)
			|| phi > self.phi_max
		{
			if t == t1 || t1 > tmax {
				return None;
			}
			t = t1;
			// compute sphere hit position and phi
			p = o + t * d;
			if p.x == 0.0 && p.y == 0.0 {
				p.x = 1e-5 * self.r;
			}
			phi = p.y.atan2(p.x);
			if phi < 0.0 {
				phi += 2.0 * PI;
			}

			if (self.z_min > -self.r && p.z < self.z_min)
				|| (self.z_max < self.r && p.z > self.z_max)
				|| phi > self.phi_max
			{
				return None;
			}
		}

		Some(QuadricIntersection { t, p, phi })
	}

	pub fn interaction_from_intersection(
		&self,
		isect: &QuadricIntersection,
		wo: Vector3f,
		time: f32,
	) -> SurfaceInteraction {
		let p = isect.p;
		let phi = isect.phi;
		let phi_max = self.phi_max;
		// find parametric representation of sphere hit
		let u = phi / phi_max;
		let cos_theta = p.z / self.r;
		let theta = safe_acos(cos_theta);
		let theta_diff = self.theta_z_max - self.theta_z_min;
		let v = (theta - self.theta_z_min) / theta_diff;
		// compute sphere dp/du and dp/dv
		let z_radius = (p.x * p.x + p.y * p.y).sqrt();
		let (cos_phi, sin_phi) = (p.x / z_radius, p.y / z_radius);
		let sin_theta = safe_sqrt(1. - cos_theta.powi(2));
		let dpdu = Vector3f::new(-phi_max * p.y, phi_max * p.x, 0.);
		let dpdv = (self.theta_z_max - self.theta_z_min)
			* Vector3f::new(p.z * cos_phi, p.z * sin_phi, -self.r * sin_theta);
		// compute sphere dn/du and dn/dv
		let d2pduu = -phi_max.powi(2) * Vector3f::new(p.x, p.y, 0.);
		let d2pduv = theta_diff * p.z * phi_max * Vector3f::new(-sin_phi, cos_phi, 0.);
		let d2pdvv = -theta_diff.powi(2) * p;
		// compute coefficients for fundamental forms
		#[allow(non_snake_case)]
		let (E, F, G) = (dpdu.dot(dpdu), dpdu.dot(dpdv), dpdv.dot(dpdv));
		let n = dpdu.cross(dpdv).normalized();
		let (e, f, g) = (n.dot(d2pduu), n.dot(d2pduv), n.dot(d2pdvv));
		// compute dn/du and dn/dv from fundamental form coefficients
		#[allow(non_snake_case)]
		let EGF2 = diff_of_products(E, G, F, F);
		#[allow(non_snake_case)]
		let inv_EGF2 = if EGF2 == 0. { 0. } else { 1. / EGF2 };
		let dndu = (f * F - e * G) * inv_EGF2 * dpdu + (e * F - f * E) * inv_EGF2 * dpdv;
		let dndv = (g * F - f * G) * inv_EGF2 * dpdu + (f * F - g * E) * inv_EGF2 * dpdv;
		// return SurfaceInteraction for quadric intersection
		let flip_normal = self.is_reversed ^ self.is_handedness_swapped;
		let wo_object = self.render_from_object.invert_vector(wo);
		let si = SurfaceInteraction::new(
			p,
			Vector2f::new(u, v),
			wo_object,
			dpdu,
			dpdv,
			dndu,
			dndv,
			time,
			flip_normal,
		);
		self.render_from_object.map_si(&si)
	}
}

impl Shape for Sphere {
	fn bounds(&self) -> Bounds3f {
		self.render_from_object.map_bounds(&Bounds3f::new(
			Vector3f::new(-self.r, -self.r, self.z_min),
			Vector3f::new(self.r, self.r, self.z_max),
		))
	}

	fn intersect(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection> {
		if let Some(isect) = self.basic_intersect(ray, tmax) {
			let intr = self.interaction_from_intersection(&isect, -ray.d, ray.time);
			Some(ShapeIntersection { intr, t: isect.t })
		} else {
			None
		}
	}

	fn intersect_p(&self, ray: &Ray, tmax: f32) -> bool {
		self.basic_intersect(ray, tmax).is_some()
	}

	fn area(&self) -> f32 {
		todo!()
	}

	#[allow(unused_variables)]
	fn sample(&self, u: Vector2f) -> Option<ShapeSample> {
		todo!()
	}

	#[allow(unused_variables)]
	fn pdf(&self, intr: &Interaction) -> f32 {
		todo!()
	}

	#[allow(unused_variables)]
	fn sample_ctx(&self, ctx: &ShapeSampleContext, u: Vector2f) -> Option<ShapeSample> {
		todo!()
	}

	#[allow(unused_variables)]
	fn pdf_ctx(&self, ctx: &ShapeSampleContext, wi: Vector3f) -> f32 {
		todo!()
	}
}

#[derive(Debug)]
pub struct QuadricIntersection {
	/// time of hit
	pub t: f32,
	/// point of hit in the object space
	pub p: Vector3f,
	pub phi: f32,
}
