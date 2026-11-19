use std::{f32::consts::PI, fmt::Debug};

use super::{
	Bounds3f, Frame, Transform, Vec2f, Vec3f,
	interaction::{Interaction, MediumInteraction, SurfaceInteraction},
	math::*,
	ray::{Ray, RayDifferential},
};

/// sin^2(1.5 deg)
const SIN2_SMALL_ANGLE: f32 = 0.00068523;

pub trait Shape: Debug {
	fn bounds(&self) -> Bounds3f;
	// TODO: fn normal_bounds(&self) -> DirectionCone;
	fn intersect(&self, ray: &Ray) -> Option<ShapeIntersection>;
	fn intersect_p(&self, ray: &Ray) -> bool;
	fn intersect_tmax(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection>;
	fn intersect_p_tmax(&self, ray: &Ray, tmax: f32) -> bool;
	fn area(&self) -> f32;
	fn sample(&self, u: Vec2f) -> Option<ShapeSample>;
	fn pdf(&self, intr: &Interaction) -> f32;
	fn sample_ctx(&self, ctx: &ShapeSampleContext, u: Vec2f) -> Option<ShapeSample>;
	/// `wi` is the incident direction
	fn pdf_ctx(&self, ctx: &ShapeSampleContext, wi: Vec3f) -> f32;
}

#[derive(Debug)]
pub struct ShapeSampleContext {
	/// reference point
	pub p: Vec3f,
	/// normal
	pub n: Vec3f,
	/// shading normal
	pub ns: Vec3f,
	/// time
	pub t: f32,
}

impl ShapeSampleContext {
	pub fn new(p: Vec3f, n: Vec3f, ns: Vec3f, t: f32) -> Self {
		Self { p, n, ns, t }
	}

	pub fn from_surface_interaction(intr: &SurfaceInteraction) -> Self {
		Self { p: intr.base.p, n: intr.base.n, ns: intr.shading.n, t: intr.base.t }
	}

	pub fn from_medium_interaction(intr: &MediumInteraction) -> Self {
		Self { p: intr.base.p, n: Vec3f::default(), ns: Vec3f::default(), t: intr.base.t }
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
	is_normal_reversed: bool,
	/// whether the transform swaps handedness
	is_handedness_swapped: bool,
}

impl Sphere {
	pub fn new(
		render_from_object: &Transform,
		is_normal_reversed: bool,
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
			is_normal_reversed,
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
		wo: Vec3f,
		time: f32,
	) -> SurfaceInteraction {
		let p = isect.p;
		let phi = isect.phi;
		let phi_max = self.phi_max;
		// find parametric representation of sphere hit
		let u = phi / phi_max;
		let cos_theta = p.z / self.r;
		let theta = safe_acos(cos_theta);
		let theta_diff = self.theta_diff();
		let v = (theta - self.theta_z_min) / theta_diff;
		// compute sphere dp/du and dp/dv
		let z_radius = (p.x * p.x + p.y * p.y).sqrt();
		let (cos_phi, sin_phi) = (p.x / z_radius, p.y / z_radius);
		let sin_theta = safe_sqrt(1. - cos_theta.powi(2));
		let dpdu = Vec3f::new(-phi_max * p.y, phi_max * p.x, 0.);
		let dpdv = theta_diff * Vec3f::new(p.z * cos_phi, p.z * sin_phi, -self.r * sin_theta);
		// compute sphere dn/du and dn/dv
		let d2pduu = -phi_max.powi(2) * Vec3f::new(p.x, p.y, 0.);
		let d2pduv = theta_diff * p.z * phi_max * Vec3f::new(-sin_phi, cos_phi, 0.);
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
		let flip_normal = self.is_normal_reversed ^ self.is_handedness_swapped;
		let wo_object = self.render_from_object.invert_vector(wo);
		let si = SurfaceInteraction::new(
			p,
			Vec2f::new(u, v),
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

	fn theta_diff(&self) -> f32 {
		self.theta_z_max - self.theta_z_min
	}
}

impl Shape for Sphere {
	fn bounds(&self) -> Bounds3f {
		self.render_from_object.map_bounds(&Bounds3f::new(
			Vec3f::new(-self.r, -self.r, self.z_min),
			Vec3f::new(self.r, self.r, self.z_max),
		))
	}

	fn intersect(&self, ray: &Ray) -> Option<ShapeIntersection> {
		self.intersect_tmax(ray, f32::INFINITY)
	}

	fn intersect_p(&self, ray: &Ray) -> bool {
		self.intersect_p_tmax(ray, f32::INFINITY)
	}

	fn intersect_tmax(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection> {
		if let Some(isect) = self.basic_intersect(ray, tmax) {
			let intr = self.interaction_from_intersection(&isect, -ray.d, ray.time);
			Some(ShapeIntersection { intr, t: isect.t })
		} else {
			None
		}
	}

	fn intersect_p_tmax(&self, ray: &Ray, tmax: f32) -> bool {
		self.basic_intersect(ray, tmax).is_some()
	}

	fn area(&self) -> f32 {
		self.phi_max * self.r * (self.z_max - self.z_min)
	}

	fn sample(&self, u: Vec2f) -> Option<ShapeSample> {
		let p_obj = self.r * sample_uniform_sphere(u);
		// compute surface normal for sphere sample and return
		let mut n = self.render_from_object.map_normal(p_obj).normalized();
		if self.is_normal_reversed {
			n *= -1.;
		}
		// compute (u,v) coordinates for sphere sample
		let theta = safe_acos(p_obj.z / self.r);
		let mut phi = p_obj.y.atan2(p_obj.x);
		if phi < 0. {
			phi += 2. * PI;
		}
		let uv = Vec2f::new(phi / self.phi_max, (theta - self.theta_z_min) / self.theta_diff());

		let p = self.render_from_object.map_point(p_obj);
		Some(ShapeSample {
			intr: Interaction { p, n, uv, ..Default::default() },
			pdf: 1. / self.area(),
		})
	}

	fn pdf(&self, _: &Interaction) -> f32 {
		1. / self.area()
	}

	fn sample_ctx(&self, ctx: &ShapeSampleContext, u: Vec2f) -> Option<ShapeSample> {
		// sample uniformly on sphere if p is inside it
		let p_center = self.render_from_object.map_point(Vec3f::zeros());
		let p_origin = ctx.offset_ray_origin_p(p_center);
		if p_origin.distance_squared(p_center) <= self.r.powi(2) {
			// sample shape by area and compute incident direction wi
			let mut ss = self.sample(u).unwrap();
			ss.intr.t = ctx.t;
			let mut wi = ss.intr.p - ctx.p;
			if wi.length_squared() == 0. {
				return None;
			}
			wi = wi.normalized();
			// convert area sample PDF in ss to solid angle measure
			ss.pdf /= ss.intr.n.abs_dot(-wi) / ctx.p.distance_squared(ss.intr.p);
			if ss.pdf.is_infinite() {
				return None;
			}

			return Some(ss);
		}

		// sample sphere uniformly inside subtended cone
		// compute quantities related to the theta_max for cone
		let sin_theta_max = self.r / ctx.p.distance(p_center);
		let sin2_theta_max = sin_theta_max.powi(2);
		let cos_theta_max = safe_sqrt(1. - sin2_theta_max);
		let mut one_minus_cos_theta_max = 1. - cos_theta_max;
		// compute theta and phi values for sample in cone
		let mut cos_theta = (cos_theta_max - 1.) * u.x + 1.;
		let mut sin2_theta = 1. - cos_theta.powi(2);
		if sin2_theta_max < SIN2_SMALL_ANGLE {
			// sin^2(1.5 deg)
			// compute cone sample via Taylor series expansion for small angles
			sin2_theta = sin2_theta_max * u.x;
			cos_theta = (1. - sin2_theta).sqrt();
			one_minus_cos_theta_max = sin2_theta_max / 2.;
		}
		// compute angle alpha from center of sphere to sampled point on surface
		let cos_alpha =
			sin2_theta / sin_theta_max + cos_theta * safe_sqrt(1. - sin2_theta / sin2_theta_max);
		let sin_alpha = safe_sqrt(1. - cos_alpha.powi(2));
		// compute surface normal and sampled point on sphere
		let phi = u.y * 2. * PI;
		let w = spherial_direction(sin_alpha, cos_alpha, phi);
		let sampling_frame = Frame::from_z((p_center - ctx.p).normalized());
		let mut n = sampling_frame.from_local(-w);
		let p = p_center + self.r * n;
		if self.is_normal_reversed {
			n = -n;
		}
		// compute p_error for sampled point on sphere
		// compute (u, v) coordinates for sampled point on sphere
		let p_obj = self.render_from_object.invert_point(p);
		let theta = safe_acos(p_obj.z / self.r);
		let mut sphere_phi = p_obj.y.atan2(p_obj.x);
		if sphere_phi < 0. {
			sphere_phi += 2. * PI;
		}
		let uv =
			Vec2f::new(sphere_phi / self.phi_max, (theta - self.theta_z_min) / self.theta_diff());

		// return ShapeSample for sampled point on sphere
		Some(ShapeSample {
			intr: Interaction { p, t: ctx.t, n, uv, ..Default::default() },
			pdf: 1. / (2. * PI * one_minus_cos_theta_max),
		})
	}

	fn pdf_ctx(&self, ctx: &ShapeSampleContext, wi: Vec3f) -> f32 {
		let p_center = self.render_from_object.map_point(Vec3f::zeros());
		let p_origin = ctx.offset_ray_origin_p(p_center);
		if p_origin.distance_squared(p_center) <= self.r.powi(2) {
			// return solid angle PDF for point inside sphere
			// intersect sample ray with shape geometry
			let ray = ctx.spawn_ray(wi);
			let Some(isect) = self.intersect(&ray) else {
				return 0.;
			};
			// compute PDF in solid angle measure from shape intersection point
			let mut pdf = (1. / self.area())
				/ (isect.intr.base.n.abs_dot(-wi) / ctx.p.distance_squared(isect.intr.base.p));
			if pdf.is_infinite() {
				pdf = 0.;
			}
			return pdf;
		}
		// compute general solid angle sphere PDF
		let sin2_theta_max = self.r.powi(2) / ctx.p.distance_squared(p_center);
		let cos_theta_max = safe_sqrt(1. - sin2_theta_max);
		let mut one_minus_cos_theta_max = 1. - cos_theta_max;
		// compute more accurate one_minus_cos_theta_max for small solid angle
		if sin2_theta_max < SIN2_SMALL_ANGLE {
			one_minus_cos_theta_max = sin2_theta_max / 2.;
		}

		1. / (2. * PI * one_minus_cos_theta_max)
	}
}

#[derive(Debug)]
pub struct QuadricIntersection {
	/// time of hit
	pub t: f32,
	/// point of hit in the object space
	pub p: Vec3f,
	pub phi: f32,
}
