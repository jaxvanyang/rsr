use minifb::Key;
use rsr::{
	Float,
	pbrt::Vector2f,
	ui::{Circle, GREEN, Result, Window},
};

fn main() -> Result<()> {
	let mut window = Window::new("Test - ESC to exit", 640, 360)?;
	let mut circle = Circle::new(50.0, 50.0, 25.0);
	let mut v = Vector2f::new(2.5, 1.5);

	window.set_target_fps(60);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		circle.c += v;
		if circle.x() + circle.r > window.width as Float || circle.x() < circle.r {
			v.x = -v.x;
		}
		if circle.y() + circle.r > window.height as Float || circle.y() < circle.r {
			v.y = -v.y;
		}

		window.clear();
		window.fill_circle(circle, GREEN);
		window.update()?;
	}

	Ok(())
}
