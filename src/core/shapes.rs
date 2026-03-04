use super::Vector2f;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
	pub position: Vector2f,
	pub width: f32,
	pub height: f32,
}

impl Rectangle {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
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
	pub radius: f32,
}

impl Circle {
	pub fn new(x: f32, y: f32, radius: f32) -> Self {
		debug_assert!(!radius.is_nan());
		Self {
			position: Vector2f::new(x, y),
			radius,
		}
	}
}
