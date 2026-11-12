use super::vecmath::Vector2i;
use std::fmt::Debug;

pub trait Film: Debug {
	fn full_resolution(&self) -> Vector2i;
}
