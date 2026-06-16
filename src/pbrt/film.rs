use super::{filter::Filter, vecmath::Vector2i};

// TODO:
pub trait Film: std::fmt::Debug {
	fn full_resolution(&self) -> Vector2i;
	fn get_filter(&self) -> &dyn Filter;
}
