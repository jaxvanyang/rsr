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

	let mut rect = Rectangle::new(0.0, 0.0, 50.0, 50.0);
	let mut v = Vector2f::new(2.0, 1.0);

	window.set_target_fps(60);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		rect.position += v;
		if rect.position.x + rect.width > buffer.width as f32 || rect.position.x < 0.0 {
			v.x = -v.x;
		}
		if rect.position.y + rect.height > buffer.height as f32 || rect.position.y < 0.0 {
			v.y = -v.y;
		}

		buffer.clear();
		buffer.fill_rect(rect, GREEN);

		window
			.update_with_buffer(&buffer, buffer.width, buffer.height)
			.unwrap();
	}
}
