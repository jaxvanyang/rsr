pub mod pbrt;
pub mod ui;

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(not(feature = "use_f64"))]
pub type Float = f32;
