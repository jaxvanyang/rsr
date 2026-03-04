use rsr::core::*;

fn main() {
	let mut buffer = ScreenBuffer::new(640, 480);
	let mut window = buffer.new_window("Rotating Cube").unwrap();
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
	let camera: f32 = 0.1;
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
		[1.0, 0.0, 0.0, buffer.width as f32 / 2.0],
		[0.0, -1.0, 0.0, buffer.height as f32 / 2.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);
	let proj = proj_screen * proj_otho * proj_camera;

	loop {
		let theta: f32 = 0.01;

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

		buffer.clear();

		for p in cube {
			let circle = Circle::new(p.x, p.y, 2.0);
			buffer.fill_circle(circle, GREEN);
		}

		buffer.update_window(&mut window).unwrap();
	}
}
