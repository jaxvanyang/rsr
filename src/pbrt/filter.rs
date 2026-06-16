use super::vecmath::Vector2f;

// TODO:
pub trait Filter: std::fmt::Debug {
	fn radius(&self) -> Vector2f;
}
