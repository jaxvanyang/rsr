use rsr::*;

fn main() {
	let mut screen = Screen::new(640, 480);
	let mut window = screen.new_window("Rotating Cube").unwrap();
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
	let camera: Float = 0.1;
	let proj_camera = SquareMatrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, camera.cos(), -camera.sin(), 0.0],
		[0.0, camera.sin(), camera.cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj_ortho = SquareMatrix::new([
		[100.0, 0.0, 0.0, 0.0],
		[0.0, 100.0, 0.0, 0.0],
		[0.0, 0.0, 100.0, -200.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj_screen = SquareMatrix::new([
		[1.0, 0.0, 0.0, screen.width as Float / 2.0],
		[0.0, -1.0, 0.0, screen.height as Float / 2.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj = Transform::new(&proj_screen * &proj_ortho * &proj_camera);

	while window.is_open() {
		let theta: Float = 0.2;
		let rotation = Transform::rotate_y(theta);
		for p in cube.iter_mut() {
			*p = rotation.map_point(*p);
		}

		let mut cube = cube;
		for p in cube.iter_mut() {
			*p = proj.map_point(*p);
		}

		screen.clear();

		for p in cube {
			let circle = Circle::new(p.x, p.y, 2.0);
			screen.fill_circle(circle, GREEN);
		}

		screen.update_window(&mut window).unwrap();
	}
}
