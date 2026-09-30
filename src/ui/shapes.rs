use crate::pbrt::vecmath::Vector2f;

/// Rectangle whose top-left corner is at `p`, with width `w` and height `h`.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
	pub p: Vector2f,
	pub w: f32,
	pub h: f32,
}

impl Rectangle {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		debug_assert!(!width.is_nan());
		debug_assert!(!height.is_nan());
		Self { p: Vector2f::new(x, y), w: width, h: height }
	}

	pub fn x(&self) -> f32 {
		self.p.x
	}

	pub fn y(&self) -> f32 {
		self.p.y
	}
}

/// Circle whose center is at `p`, with radius `r`.
#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub c: Vector2f,
	pub r: f32,
}

impl Circle {
	pub fn new(x: f32, y: f32, radius: f32) -> Self {
		debug_assert!(!radius.is_nan());
		Self { c: Vector2f::new(x, y), r: radius }
	}

	pub fn x(&self) -> f32 {
		self.c.x
	}

	pub fn y(&self) -> f32 {
		self.c.y
	}
}
