use super::{Float, Vector2f};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
	pub position: Vector2f,
	pub width: Float,
	pub height: Float,
}

impl Rectangle {
	pub fn new(x: Float, y: Float, width: Float, height: Float) -> Self {
		debug_assert!(!width.is_nan());
		debug_assert!(!height.is_nan());
		Self {
			position: Vector2f::new(x, y),
			width,
			height,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub position: Vector2f,
	pub radius: Float,
}

impl Circle {
	pub fn new(x: Float, y: Float, radius: Float) -> Self {
		debug_assert!(!radius.is_nan());
		Self {
			position: Vector2f::new(x, y),
			radius,
		}
	}
}
