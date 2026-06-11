use minifb::{Key, KeyRepeat};
use rsr::{
	Float,
	pbrt::{
		Vector2f, Vector2i,
		color::{RGB, XYZ},
		colorspace::{self as cs, RGBColorSpace},
		spectrum::{DenselySampledSpectrum, LAMBDA_MAX_I, LAMBDA_MIN_I, Spectrum},
	},
	ui::{Result, Window, color},
};

fn main() -> Result<()> {
	let mut window = Window::new("Chromaticity Diagram", 500, 500)?;
	let mut boundary = Vec::new();
	for lambda in LAMBDA_MIN_I..=LAMBDA_MAX_I {
		let spectrum = new_light(lambda, 1.);
		let xy = XYZ::from(&spectrum as &dyn Spectrum).xy();
		boundary.push(to_screen(xy, window.width, window.height));
	}
	let colorspaces = [&cs::sRGB, &cs::DCI_P3, &cs::Rec2020, &cs::ACES2065_1];
	let mut i = 0;

	while window.is_open() {
		if window.is_key_pressed(Key::Space, KeyRepeat::No) {
			i = (i + 1) % colorspaces.len();
		}

		draw_diagram(&mut window, &boundary, colorspaces[i]);

		window.update()?;
	}

	Ok(())
}

fn draw_diagram(window: &mut Window, boundary: &[Vector2i], cs: &RGBColorSpace) {
	window.fill(color::GRAY_A);
	window.fill_polygon(boundary, color::BLACK);

	let w = (window.width - 1) as Float;
	let h = (window.height - 1) as Float;
	for j in 0..window.height {
		for i in 0..window.width {
			if window[(i, j)] != color::BLACK {
				continue;
			}

			let x = i as Float / w;
			let y = (window.height - j) as Float / h;
			let xyz = XYZ::from_xy(Vector2f::new(x, y));
			let rgb = cs.to_rgb(xyz);

			window[(i, j)] = rgb.into();
		}
	}

	let r = cs.to_xyz(RGB::RED).xy();
	let g = cs.to_xyz(RGB::GREEN).xy();
	let b = cs.to_xyz(RGB::BLUE).xy();
	let f = |p| to_screen(p, window.width, window.height);
	window.draw_triangle(f(r), f(g), f(b), color::BLACK);
}

fn to_screen(point: Vector2f, width: usize, height: usize) -> Vector2i {
	let w = (width - 1) as Float;
	let h = (height - 1) as Float;
	let x = point.x * w;
	let y = (1. - point.y) * h;

	Vector2f::new(x, y).into()
}

fn new_light(lambda: usize, intensity: Float) -> DenselySampledSpectrum {
	DenselySampledSpectrum::new_with_values(lambda, &[intensity])
}
