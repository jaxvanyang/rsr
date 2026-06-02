use rsr::{
	Float,
	pbrt::{SquareMatrix, Transform, Vector3f},
	ui::{Circle, GREEN, Result, Window},
};

fn main() -> Result<()> {
	let mut window = Window::new("Rotating Cube", 640, 480)?;
	let mut cube = [
		Vector3f::new(1.0, 1.0, 1.0),
		Vector3f::new(1.0, 1.0, -1.0),
		Vector3f::new(-1.0, 1.0, -1.0),
		Vector3f::new(-1.0, 1.0, 1.0),
		Vector3f::new(1.0, -1.0, 1.0),
		Vector3f::new(1.0, -1.0, -1.0),
		Vector3f::new(-1.0, -1.0, -1.0),
		Vector3f::new(-1.0, -1.0, 1.0),
	];
	let world2camera = Transform::look_at(
		Vector3f::new(10.0, 10.0, 0.0),
		Vector3f::new(0.0, 0.0, 0.0),
		Vector3f::new(0.0, 1.0, 0.0),
	);
	let proj_ortho = SquareMatrix::from([
		[100.0, 0.0, 0.0, 0.0],
		[0.0, 100.0, 0.0, 0.0],
		[0.0, 0.0, 100.0, -200.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj_screen = SquareMatrix::from([
		[1.0, 0.0, 0.0, window.width as Float / 2.0],
		[0.0, -1.0, 0.0, window.height as Float / 2.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj = Transform::new(&proj_screen * &proj_ortho * world2camera.get_matrix());

	let rotation = Transform::rotate_y(0.2);
	while window.is_open() {
		window.clear();

		for i in cube.iter_mut() {
			let p = proj.map_point(*i);
			let circle = Circle::new(p.x, p.y, 2.0);
			window.fill_circle(circle, GREEN);

			*i = rotation.map_point(*i);
		}

		window.update()?;
	}

	Ok(())
}
