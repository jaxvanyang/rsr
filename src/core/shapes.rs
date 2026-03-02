use super::Vector2f;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
	pub position: Vector2f,
	pub width: f32,
	pub height: f32,
}

impl Rectangle {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			position: Vector2f::new(x, y),
			width,
			height,
		}
	}
}
