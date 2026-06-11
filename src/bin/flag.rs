use rsr::{
	Float,
	pbrt::{SquareMatrix, Vector2f, math::PI},
	ui::{Rectangle, Result, Window, elapsed_with_update},
};
use std::time::Instant;

fn main() -> Result<()> {
	let mut window = Window::new("Flag", 800, 600)?;
	let mut width = 300.0;
	let mut t0 = Instant::now();

	while window.is_open() {
		let dt = elapsed_with_update(&mut t0);
		width += dt * 50.;

		draw_china_flag(&mut window, width);

		window.update()?;
	}

	Ok(())
}

fn draw_china_flag(window: &mut Window, width: Float) {
	let height = width * 2. / 3.;
	let x = (window.width as Float - width) / 2.;
	let y = (window.height as Float - height) / 2.;
	let unit = width / 30.;
	let rect = Rectangle::new(x, y, width, height);
	let red = 0xee1c25;

	window.clear();
	window.fill_rect(rect, red);
	draw_star(window, x + unit * 5., y + unit * 5., unit * 3., 0.);
	draw_star(window, x + unit * 10., y + unit * 2., unit, Float::atan2(5., 3.) + PI);
	draw_star(window, x + unit * 12., y + unit * 4., unit, Float::atan2(7., 1.) + PI);
	draw_star(window, x + unit * 12., y + unit * 7., unit, Float::atan2(7., -2.) + PI);
	draw_star(window, x + unit * 10., y + unit * 9., unit, Float::atan2(5., -4.) + PI);
}

fn draw_star(window: &mut Window, x: Float, y: Float, radius: Float, theta: Float) {
	let yellow = 0xffff00;
	let center = Vector2f::new(x, y);
	let (sin, cos) = theta.sin_cos();
	let rotation = SquareMatrix::from([[cos, -sin], [sin, cos]]);
	let inner_radius = radius * Float::to_radians(18.).sin() / Float::to_radians(126.).sin();
	let mut points = [Vector2f::default(); 10];
	for i in 0..5 {
		let t0 = i as Float * 72. - 90.;
		let t1 = i as Float * 72. - 54.;
		let (sin0, cos0) = t0.to_radians().sin_cos();
		let (sin1, cos1) = t1.to_radians().sin_cos();
		points[i * 2] = Vector2f::new(cos0, sin0) * radius;
		points[i * 2 + 1] = Vector2f::new(cos1, sin1) * inner_radius;
	}
	for p in points.iter_mut() {
		*p = center + &rotation * *p;
	}

	window.fill_polygon(&points.map(|p| p.into()), yellow);
}
