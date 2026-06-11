pub mod camera;
pub mod color;
pub mod colorspace;
pub mod interaction;
pub mod math;
pub mod number;
pub mod spectrum;
pub mod transform;
pub mod vecmath;

pub use camera::*;
pub use transform::*;
pub use vecmath::*;

use crate::Float;

// TODO: trait HasNAN
