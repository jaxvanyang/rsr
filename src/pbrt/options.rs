pub static mut OPTIONS: Options =
	Options { rendering_space: RenderingCoordinateSystem::CameraWorld };

#[derive(Debug)]
pub struct Options {
	pub rendering_space: RenderingCoordinateSystem,
}

#[derive(Debug)]
pub enum RenderingCoordinateSystem {
	Camera,
	CameraWorld,
	World,
}
