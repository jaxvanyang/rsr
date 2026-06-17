use minifb::Key;
use rsr::{
	Float,
	pbrt::Vector2f,
	ui::{Circle, Result, Window, color},
};

fn main() -> Result<()> {
	let mut window = Window::new("Test - ESC to exit", 640, 360)?;
	let mut circle = Circle::new(50.0, 50.0, 25.0);
	let mut v = Vector2f::new(250., 150.);
	let colors = [color::RED, color::GREEN, color::BLUE];
	let mut c = 0;

	window.set_target_fps(0);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		circle.c += v * window.delta_time();
		if circle.x() + circle.r > window.width as Float || circle.x() < circle.r {
			v.x = -v.x;
			c = (c + 1) % colors.len();
		}
		if circle.y() + circle.r > window.height as Float || circle.y() < circle.r {
			v.y = -v.y;
			c = (c + 1) % colors.len();
		}

		window.clear();
		window.fill_circle(circle, colors[c]);
		window.draw_fps(2, 2);
		window.update()?;
	}

	Ok(())
}
