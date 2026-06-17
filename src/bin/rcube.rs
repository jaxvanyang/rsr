use std::time::Instant;

use rsr::{
	Float,
	pbrt::{SquareMatrix, Transform, Vector2i, Vector3f},
	ui::{Result, Window, color, elapsed_with_update},
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
	let proj = Transform::new(proj_screen * proj_ortho * world2camera.get_matrix());
	let mut t0 = Instant::now();

	while window.is_open() {
		let dt = elapsed_with_update(&mut t0);
		let rotation = Transform::rotate_y(dt * 30.);
		for p in cube.iter_mut() {
			*p = rotation.map_point(*p);
		}

		draw_cube(&mut window, &cube, &proj);
		window.draw_fps(2, 2);

		window.update()?;
	}

	Ok(())
}

fn draw_cube(window: &mut Window, cube: &[Vector3f; 8], projection: &Transform) {
	let cube = cube
		.map(|p| {
			let q = projection.map_point(p);
			Vector2i::new(q.x as i32, q.y as i32)
		})
		.into_iter()
		.collect::<Vec<_>>();

	window.clear();
	window.draw_polygon(&cube[..4], color::BLUE);
	window.draw_polygon(&cube[4..], color::BLUE);
	for (i, p) in cube.iter().enumerate().take(4) {
		let q = cube[i + 4];
		window.draw_line(p.x, p.y, q.x, q.y, color::BLUE);
	}
}
