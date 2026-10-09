//! A simple ray tracer demo.
//!
//! References:
//! - https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-ray-tracing//how-does-it-work.html
//! - https://github.com/scratchapixel/scratchapixel-code/blob/main/introduction-to-ray-tracing/raytracer.cpp

use std::{f32::consts::PI, thread};

use anyhow::Result;
use approx::assert_abs_diff_eq;
use minifb::Key;
use rsr::{
	pbrt::{math::lerp, *},
	ui::{color, *},
};

struct Game {
	pub spheres: Vec<Sphere>,
	pub world_to_camera: Transform,
	pub pos: Vector3f,
	pub x_rot: f32,
	pub y_rot: f32,
}

struct Sphere {
	pub center: Vector3f,
	#[allow(dead_code)]
	pub radius: f32,
	pub radius2: f32,
	pub surface_color: Vector3f,
	pub emission_color: Vector3f,
	pub transparency: f32,
	pub reflection: f32,
}

struct SphereBuilder(Sphere);

fn main() -> Result<()> {
	let mut game = Game::new();

	let mut window = Window::new("Ray Tracer", 640, 480)?;
	window.set_target_fps(0);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		window.enable_screenshot()?;

		update(&window, &mut game);
		draw(&mut window, &game)?;
	}

	Ok(())
}

fn update(window: &Window, game: &mut Game) {
	let dt = window.delta_time();
	let d_pos = dt * 5.0;
	let d_rot = dt * 15.0;
	let y_rot = Transform::rotate_y(game.y_rot);
	let x = y_rot.map_vector(Vector3f::new(1.0, 0.0, 0.0));
	let x_rot = Transform::rotate(x, game.x_rot);
	let z = y_rot.map_vector(Vector3f::new(0.0, 0.0, 1.0));

	if window.is_key_down(Key::Space) {
		game.pos.y += d_pos;
	}
	if window.is_key_down(Key::LeftShift) {
		game.pos.y -= d_pos;
	}
	if window.is_key_down(Key::W) {
		game.pos -= z * d_pos;
	}
	if window.is_key_down(Key::S) {
		game.pos += z * d_pos;
	}
	if window.is_key_down(Key::A) {
		game.pos -= x * d_pos;
	}
	if window.is_key_down(Key::D) {
		game.pos += x * d_pos;
	}

	if window.is_key_down(Key::Up) {
		game.x_rot -= d_rot;
	}
	if window.is_key_down(Key::Down) {
		game.x_rot += d_rot;
	}
	if window.is_key_down(Key::Left) {
		game.y_rot += d_rot;
	}
	if window.is_key_down(Key::Right) {
		game.y_rot -= d_rot;
	}

	game.world_to_camera = (x_rot * y_rot).inv().unwrap() * Transform::translate(-game.pos);
}

impl Game {
	pub fn new() -> Self {
		Self {
			spheres: vec![
				Sphere::builder(
					Vector3f::new(0.0, -10004.0, -20.0),
					10000.0,
					Vector3f::new(0.2, 0.2, 0.2),
				)
				.build(),
				Sphere::builder(
					Vector3f::new(0.0, 0.0, -20.0),
					4.0,
					Vector3f::new(1.00, 0.32, 0.36),
				)
				.reflection(1.0)
				.transparency(0.5)
				.build(),
				Sphere::builder(
					Vector3f::new(5.0, -1.0, -15.0),
					2.0,
					Vector3f::new(0.90, 0.76, 0.46),
				)
				.reflection(1.0)
				.build(),
				Sphere::builder(
					Vector3f::new(5.0, 0.0, -25.0),
					3.0,
					Vector3f::new(0.65, 0.77, 0.97),
				)
				.reflection(1.0)
				.build(),
				Sphere::builder(Vector3f::new(-5.5, 0.0, -15.0), 3.0, Vector3f::ones())
					.reflection(1.0)
					.build(),
				// light
				Sphere::builder(Vector3f::new(0.0, 20.0, -30.0), 3.0, Vector3f::zeros())
					.emission_color(Vector3f::sames(3.0))
					.build(),
			],
			world_to_camera: Transform::default(),
			pos: Vector3f::zeros(),
			x_rot: 0.0,
			y_rot: 0.0,
		}
	}
}

fn draw(window: &mut Window, game: &Game) -> Result<()> {
	render(&mut window.buffer, game);
	window.draw_fps(2, 2);
	window.draw_frame_time(2, 16);
	window.draw_text(&format!("pos: {}", game.pos), 2, 30, 2, color::GREEN);
	window.draw_text(&format!("x_rot: {}", game.x_rot), 2, 44, 2, color::GREEN);
	window.draw_text(&format!("y_rot: {}", game.y_rot), 2, 58, 2, color::GREEN);

	window.update()?;
	Ok(())
}

fn render(buffer: &mut ScreenBuffer, game: &Game) {
	let w = buffer.w();
	let h = buffer.h();
	let invw = 1.0 / w as f32;
	let invh = 1.0 / h as f32;
	let fov = 30.0;
	let aspectratio = w as f32 / h as f32;
	let angle = (PI * 0.5 * fov / 180.0).tan();
	let screen_to_camera = SquareMatrix::<4>::from([
		[2.0 * invw * angle * aspectratio, 0.0, 0.0, (invw - 1.0) * angle * aspectratio],
		[0.0, -2.0 * invh * angle, 0.0, (-invh + 1.0) * angle],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let spheres: Vec<Sphere> =
		game.spheres.iter().map(|s| s.apply(&game.world_to_camera)).collect();

	// trace rays
	let n_thread = 8;
	let rows_per_thread = h.div_ceil(n_thread);
	let buffer = buffer.buffer.as_mut_slice();
	thread::scope(|s| {
		for (t, chunk) in buffer.chunks_mut(rows_per_thread * w).enumerate() {
			let screen_to_camera = &screen_to_camera;
			let spheres = &spheres;

			s.spawn(move || {
				for (i, row) in chunk.chunks_mut(w).enumerate() {
					let y = t * rows_per_thread + i;
					for (x, pixel) in row.iter_mut().enumerate() {
						let dir = Vector3f::new(x as f32, y as f32, -1.0);
						let dir = screen_to_camera.mul_point(dir).normalized();
						*pixel = to_color(trace(Vector3f::new(0.0, 0.0, 0.0), dir, spheres, 5));
					}
				}
			});
		}
	});
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
	// change t to tweak the effect
	let fresneleffect = lerp((1.0 - facingratio).powi(3), 1.0, 0.1);
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

	let v = reflection * fresneleffect + refraction * (1.0 - fresneleffect) * sphere.transparency;
	let surface_color = sphere.surface_color * v;

	surface_color + sphere.emission_color
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

	pub fn apply(&self, m: &Transform) -> Self {
		Self { center: m.map_point(self.center), ..*self }
	}
}

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
