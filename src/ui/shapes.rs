use crate::{Float, pbrt::vecmath::Vector2f};

/// Rectangle whose top-left corner is at `p`, with width `w` and height `h`.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
	pub p: Vector2f,
	pub w: Float,
	pub h: Float,
}

impl Rectangle {
	pub fn new(x: Float, y: Float, width: Float, height: Float) -> Self {
		debug_assert!(!width.is_nan());
		debug_assert!(!height.is_nan());
		Self {
			p: Vector2f::new(x, y),
			w: width,
			h: height,
		}
	}

	pub fn x(&self) -> Float {
		self.p.x
	}

	pub fn y(&self) -> Float {
		self.p.y
	}
}

/// Circle whose center is at `p`, with radius `r`.
#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub c: Vector2f,
	pub r: Float,
}

impl Circle {
	pub fn new(x: Float, y: Float, radius: Float) -> Self {
		debug_assert!(!radius.is_nan());
		Self {
			c: Vector2f::new(x, y),
			r: radius,
		}
	}

	pub fn x(&self) -> Float {
		self.c.x
	}

	pub fn y(&self) -> Float {
		self.c.y
	}
}
