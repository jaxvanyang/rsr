//! A simple ray tracer demo.
//!
//! References:
//! - https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-ray-tracing//how-does-it-work.html
//! - https://github.com/scratchapixel/scratchapixel-code/blob/main/introduction-to-ray-tracing/raytracer.cpp

use std::f32::consts::PI;

use anyhow::Result;
use approx::assert_abs_diff_eq;
use minifb::{Key, KeyRepeat};
use rsr::{pbrt::*, ui::*};

fn main() -> Result<()> {
	let spheres = vec![
		Sphere::builder(Vector3f::new(0.0, -10004.0, -20.0), 10000.0, Vector3f::new(0.2, 0.2, 0.2))
			.build(),
		Sphere::builder(Vector3f::new(0.0, 0.0, -20.0), 4.0, Vector3f::new(1.00, 0.32, 0.36))
			.reflection(1.0)
			.transparency(0.5)
			.build(),
		Sphere::builder(Vector3f::new(5.0, -1.0, -15.0), 2.0, Vector3f::new(0.90, 0.76, 0.46))
			.reflection(1.0)
			.build(),
		Sphere::builder(Vector3f::new(5.0, 0.0, -25.0), 3.0, Vector3f::new(0.65, 0.77, 0.97))
			.reflection(1.0)
			.build(),
		Sphere::builder(Vector3f::new(-5.5, 0.0, -15.0), 3.0, Vector3f::new(1.00, 1.00, 1.00))
			.reflection(1.0)
			.build(),
		// light
		Sphere::builder(Vector3f::new(0.0, 20.0, -30.0), 3.0, Vector3f::new(0.00, 0.00, 0.00))
			.emission_color(Vector3f::new(3.0, 3.0, 3.0))
			.build(),
	];

	let mut window = Window::new("Ray Tracer", 640, 480)?;
	window.set_target_fps(0);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		if window.is_key_down(Key::LeftCtrl) && window.is_key_pressed(Key::S, KeyRepeat::No) {
			window.take_screenshot()?;
		}

		draw(&mut window, &spheres)?;
	}

	Ok(())
}

fn draw(window: &mut Window, spheres: &[Sphere]) -> Result<()> {
	render(&mut window.buffer, &spheres);
	window.draw_fps(2, 2);

	window.update()?;
	Ok(())
}

fn render(buffer: &mut ScreenBuffer, spheres: &[Sphere]) {
	let w = buffer.w();
	let h = buffer.h();
	let invw = 1.0 / w as f32;
	let invh = 1.0 / h as f32;
	let fov = 30.0;
	let aspectratio = w as f32 / h as f32;
	let angle = PI * 0.5 * fov / 180.0;

	// trace rays
	let mut i = 0;
	for y in 0..h {
		for x in 0..w {
			let x_ = (2.0 * ((x as f32 + 0.5) * invw) - 1.0) * angle * aspectratio;
			let y_ = (1.0 - 2.0 * ((y as f32 + 0.5) * invh)) * angle;
			let dir = Vector3f::new(x_, y_, -1.0).normalized();
			buffer.buffer[i] = to_color(trace(Vector3f::new(0.0, 0.0, 0.0), dir, spheres, 5));
			i += 1;
		}
	}
}

fn to_color(v: Vector3f) -> u32 {
	let r = (v.x.clamp(0.0, 1.0) * 255.0) as u8;
	let g = (v.y.clamp(0.0, 1.0) * 255.0) as u8;
	let b = (v.z.clamp(0.0, 1.0) * 255.0) as u8;
	u32::from_rgb(r, g, b)
}

fn trace(rayorig: Vector3f, raydir: Vector3f, spheres: &[Sphere], depth: u32) -> Vector3f {
	#[cfg(debug_assertions)]
	assert_abs_diff_eq!(raydir.length(), 1.0);

	let mut tnear = f32::MAX;
	let mut sphere: Option<&Sphere> = None;
	// find intersection of this ray with the sphere in the scene
	for s in spheres {
		if let Some(t) = s.intersect(rayorig, raydir)
			&& t < tnear
		{
			tnear = t;
			sphere = Some(s);
		}
	}

	if sphere.is_none() {
		return Vector3f::new(1.0, 1.0, 1.0);
	}

	let sphere = sphere.unwrap();
	if depth == 0 {
		return sphere.emission_color;
	}

	let mut surface_color = Vector3f::default();
	let phit = rayorig + raydir * tnear;
	let bias = 1e-4; // add some bias to the point from which we will be tracing
	let mut nhit = (phit - sphere.center).normalized();
	let inside = if raydir.dot(nhit) > 0.0 {
		nhit = -nhit;
		true
	} else {
		false
	};
	let facingratio = -raydir.dot(nhit);
	// change the mix value to tweak the effect
	let fresneleffect = lerp(0.1, (1.0 - facingratio).powi(3), 1.0);
	let refldir = raydir - nhit * 2.0 * raydir.dot(nhit);
	#[cfg(debug_assertions)]
	assert_abs_diff_eq!(refldir.length(), 1.0);
	let reflection = trace(phit + nhit * bias, refldir, spheres, depth - 1);
	let refraction = if sphere.transparency > 0.0 {
		let ior = 1.1;
		let cosi = -nhit.dot(raydir);
		let eta = if inside { ior } else { 1.0 / ior };
		let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
		let refrdir = (raydir * eta + nhit * (eta * cosi - k.sqrt())).normalized();

		trace(phit - nhit * bias, refrdir, spheres, depth - 1)
	} else {
		Vector3f::default()
	};

	// TODO: vector component-wise multiplication
	let v = reflection * fresneleffect + refraction * (1.0 - fresneleffect) * sphere.transparency;
	surface_color.x = sphere.surface_color.x * v.x;
	surface_color.y = sphere.surface_color.y * v.y;
	surface_color.z = sphere.surface_color.z * v.z;

	surface_color + sphere.emission_color
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
	a + t * (b - a)
}

struct Sphere {
	pub center: Vector3f,
	pub radius: f32,
	pub radius2: f32,
	pub surface_color: Vector3f,
	pub emission_color: Vector3f,
	pub transparency: f32,
	pub reflection: f32,
}

impl Sphere {
	pub fn new(center: Vector3f, radius: f32, surface_color: Vector3f) -> Self {
		Sphere {
			center,
			radius,
			radius2: radius * radius,
			surface_color,
			emission_color: Vector3f::default(),
			transparency: 0.0,
			reflection: 0.0,
		}
	}

	pub fn builder(center: Vector3f, radius: f32, surface_color: Vector3f) -> SphereBuilder {
		let sphere = Sphere::new(center, radius, surface_color);

		SphereBuilder(sphere)
	}

	pub fn intersect(&self, rayorig: Vector3f, raydir: Vector3f) -> Option<f32> {
		let l = self.center - rayorig;
		let tca = l.dot(raydir);
		if tca < 0.0 {
			return None;
		}
		let d2 = l.length_squared() - tca * tca;
		if d2 > self.radius2 {
			return None;
		}
		let thc = (self.radius2 - d2).sqrt();
		let t0 = tca - thc;
		let t1 = tca + thc;

		if t0 >= 0.0 {
			Some(t0)
		} else if t1 >= 0.0 {
			Some(t1)
		} else {
			None
		}
	}
}

struct SphereBuilder(Sphere);

impl SphereBuilder {
	pub fn build(self) -> Sphere {
		self.0
	}

	pub fn emission_color(mut self, emission_color: Vector3f) -> Self {
		self.0.emission_color = emission_color;
		self
	}

	pub fn transparency(mut self, transparency: f32) -> Self {
		self.0.transparency = transparency;
		self
	}

	pub fn reflection(mut self, reflection: f32) -> Self {
		self.0.reflection = reflection;
		self
	}
}
