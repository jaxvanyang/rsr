use minifb::{Key, Window, WindowOptions};
use rsr::*;

fn main() {
	let mut buffer = ScreenBuffer::new(640, 360);
	let mut window = Window::new(
		"Test - ESC to exit",
		buffer.width,
		buffer.height,
		WindowOptions::default(),
	)
	.unwrap();

	let mut circle = Circle::new(50.0, 50.0, 25.0);
	let mut v = Vector2f::new(2.5, 1.5);

	window.set_target_fps(60);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		circle.position += v;
		if circle.position.x + circle.radius > buffer.width as f32
			|| circle.position.x < circle.radius
		{
			v.x = -v.x;
		}
		if circle.position.y + circle.radius > buffer.height as f32
			|| circle.position.y < circle.radius
		{
			v.y = -v.y;
		}

		buffer.clear();
		buffer.fill_circle(circle, GREEN);

		window
			.update_with_buffer(&buffer, buffer.width, buffer.height)
			.unwrap();
	}
}
