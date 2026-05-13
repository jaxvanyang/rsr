use super::{Float, Vector3f};

pub struct Camera {
	pub position: Vector3f,
	pub look_at: Vector3f,
	pub up: Vector3f,
	pub fov: Float,
}
