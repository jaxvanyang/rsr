use rsr::core::*;

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
	let proj_camera = Matrix44f([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, camera.cos(), -camera.sin(), 0.0],
		[0.0, camera.sin(), camera.cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj_otho = Matrix44f([
		[100.0, 0.0, 0.0, 0.0],
		[0.0, 100.0, 0.0, 0.0],
		[0.0, 0.0, 100.0, -200.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj_screen = Matrix44f([
		[1.0, 0.0, 0.0, screen.width as Float / 2.0],
		[0.0, -1.0, 0.0, screen.height as Float / 2.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj = proj_screen * proj_otho * proj_camera;

	while window.is_open() {
		let theta: Float = 0.01;

		let rotation = Matrix44f([
			[theta.cos(), 0.0, -theta.sin(), 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[theta.sin(), 0.0, theta.cos(), 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		for p in cube.iter_mut() {
			*p = rotation.mul_point(*p);
		}

		let mut cube = cube;
		for p in cube.iter_mut() {
			*p = proj.mul_point(*p);
		}

		screen.clear();

		for p in cube {
			let circle = Circle::new(p.x, p.y, 2.0);
			screen.fill_circle(circle, GREEN);
		}

		screen.update_window(&mut window).unwrap();
	}
}
