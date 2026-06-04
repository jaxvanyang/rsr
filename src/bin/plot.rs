use rsr::{
	Float,
	pbrt::{
		SquareMatrix, Vector2f, Vector2i,
		spectrum::{Spectrum, spectra},
	},
	ui::{Result, Window, color},
};

fn main() -> Result<()> {
	let mut window = Window::new("Plot", 800, 600)?;
	let center = Vector2i::new(-200, window.height as i32 / 2);

	while window.is_open() {
		draw_background(&mut window, center, 50);
		plot(&mut window, center, |x| spectra::x().eval(x), color::RED);
		plot(&mut window, center, |x| spectra::y().eval(x), color::GREEN);
		plot(&mut window, center, |x| spectra::z().eval(x), color::BLUE);

		window.update()?;
	}

	Ok(())
}

fn plot(window: &mut Window, center: Vector2i, f: fn(Float) -> Float, color: u32) {
	let flip = SquareMatrix::<2>::from([[1., 0.], [0., -1.]]);
	let scale = SquareMatrix::<2>::from([[1., 0.], [0., 110.]]);
	let transform = flip * scale;
	let offset = Vector2f::from(center);
	for x in 360..=830 {
		let x = x as Float;
		let p = Vector2f::new(x, f(x));
		let p = &transform * p + offset;
		window.draw_pixel_vf(p, color);
	}
	println!();
}

fn draw_background(window: &mut Window, center: Vector2i, step: usize) {
	let (x, y) = (center.x, center.y);
	let step_i = step as i32;
	let x_min = if x < 0 {
		(x % step_i) + step_i
	} else {
		x % step_i
	} as usize;
	let y_min = if y < 0 {
		(y % step_i) + step_i
	} else {
		y % step_i
	} as usize;

	window.clear();
	for y in (y_min..window.height).step_by(step) {
		for x in (x_min..window.width).step_by(step) {
			window[(x, y)] = color::BLUE_D;
		}
	}
	window.draw_line(x, 0, x, window.height as i32, color::BLUE_E);
	window.draw_line(0, y, window.width as i32, y, color::BLUE_E);
}
