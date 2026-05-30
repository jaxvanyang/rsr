pub mod camera;
pub mod pbrt;
pub mod screen;
pub mod shapes;

pub use camera::*;
pub use pbrt::*;
pub use screen::*;
pub use shapes::*;

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(not(feature = "use_f64"))]
pub type Float = f32;
