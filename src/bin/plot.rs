use rsr::{
	pbrt::{
		SquareMatrix, Vector2f, Vector2i,
		spectrum::{Spectrum, spectra},
	},
	ui::{Canvas, Result, Window, color::manim::*},
};

fn main() -> Result<()> {
	let mut window = Window::new("Plot", 800, 600)?;
	let center = Vector2i::new(-200, window.h() as i32 / 2);

	while window.is_open() {
		draw_background(&mut window, center, 50);
		plot(&mut window, center, |x| spectra::x().eval(x), RED);
		plot(&mut window, center, |x| spectra::y().eval(x), GREEN);
		plot(&mut window, center, |x| spectra::z().eval(x), BLUE);

		window.update()?;
	}

	Ok(())
}

fn plot(window: &mut Window, center: Vector2i, f: fn(f32) -> f32, color: u32) {
	let flip = SquareMatrix::<2>::from([[1., 0.], [0., -1.]]);
	let scale = SquareMatrix::<2>::from([[1., 0.], [0., 110.]]);
	let transform = flip * scale;
	let offset = Vector2f::from(center);
	let mut points = Vec::new();
	for x in 360..=830 {
		let x = x as f32;
		points.push(&transform * Vector2f::new(x, f(x)) + offset);
	}
	window.draw_lines(&points, color);
}

fn draw_background(window: &mut Window, center: Vector2i, step: usize) {
	let (x, y) = (center.x, center.y);
	let step_i = step as i32;
	let x_min = if x < 0 { (x % step_i) + step_i } else { x % step_i } as usize;
	let y_min = if y < 0 { (y % step_i) + step_i } else { y % step_i } as usize;

	window.clear();
	for y in (y_min..window.h()).step_by(step) {
		for x in (x_min..window.w()).step_by(step) {
			window[(x, y)] = BLUE_D;
		}
	}
	window.draw_line(x, 0, x, window.h() as i32, BLUE_E);
	window.draw_line(0, y, window.w() as i32, y, BLUE_E);
}
