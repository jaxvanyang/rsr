use super::{Bounds3f, Transform, ray::Ray, shape::*};
use std::fmt::Debug;

pub trait Primitive: Debug {
	fn bounds(&self) -> Bounds3f;
	fn intersect_tmax(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection>;
	fn intersect_p_tmax(&self, ray: &Ray, tmax: f32) -> bool;

	fn intersect(&self, ray: &Ray) -> Option<ShapeIntersection> {
		self.intersect_tmax(ray, f32::INFINITY)
	}
	fn intersect_p(&self, ray: &Ray) -> bool {
		self.intersect_p_tmax(ray, f32::INFINITY)
	}
}

#[derive(Debug)]
pub struct GeometricPrimitive {
	shape: Box<dyn Shape>,
	// TODO: material, area_light, medium_interface, alpha
}

impl GeometricPrimitive {
	pub fn new(shape: impl Shape + 'static) -> Self {
		Self { shape: Box::new(shape) }
	}
}

impl Primitive for GeometricPrimitive {
	fn bounds(&self) -> Bounds3f {
		self.shape.bounds()
	}

	fn intersect_tmax(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection> {
		let si = self.shape.intersect_tmax(ray, tmax)?;
		// TODO: test intersection against alpha texture, if present
		// TODO: initialize SurfaceInteraction after Shape intersection

		Some(si)
	}

	fn intersect_p_tmax(&self, ray: &Ray, tmax: f32) -> bool {
		// TODO: check alpha
		self.shape.intersect_p_tmax(ray, tmax)
	}
}

#[derive(Debug)]
pub struct SimplePrimitive {
	shape: Box<dyn Shape>,
	// TODO: material
}

impl SimplePrimitive {
	pub fn new(shape: impl Shape + 'static) -> Self {
		Self { shape: Box::new(shape) }
	}
}

impl Primitive for SimplePrimitive {
	fn bounds(&self) -> Bounds3f {
		self.shape.bounds()
	}

	fn intersect_tmax(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection> {
		let si = self.shape.intersect_tmax(ray, tmax)?;
		// TODO: initialize SurfaceInteraction after Shape intersection

		Some(si)
	}

	fn intersect_p_tmax(&self, ray: &Ray, tmax: f32) -> bool {
		self.shape.intersect_p_tmax(ray, tmax)
	}
}

#[derive(Debug)]
pub struct TransformedPrimitive<'a> {
	primitive: Box<dyn Primitive>,
	render_from_primitive: &'a Transform,
}

impl<'a> TransformedPrimitive<'a> {
	pub fn new(primitive: impl Primitive + 'static, render_from_primitive: &'a Transform) -> Self {
		Self { primitive: Box::new(primitive), render_from_primitive }
	}
}

impl<'a> Primitive for TransformedPrimitive<'a> {
	fn bounds(&self) -> Bounds3f {
		self.render_from_primitive.map_bounds(&self.primitive.bounds())
	}

	fn intersect_tmax(&self, ray: &Ray, tmax: f32) -> Option<ShapeIntersection> {
		// transform ray to primitive-space and intersect with primitive
		let ray = self.render_from_primitive.invert_ray(ray);
		let mut si = self.primitive.intersect_tmax(&ray, tmax)?;
		si.intr = self.render_from_primitive.map_si(&si.intr);

		Some(si)
	}

	fn intersect_p_tmax(&self, ray: &Ray, tmax: f32) -> bool {
		self.primitive.intersect_p_tmax(ray, tmax)
	}
}

// TODO: AnimatedPrimitive
