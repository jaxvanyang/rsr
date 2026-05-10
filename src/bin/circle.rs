use minifb::Key;
use rsr::*;

fn main() {
	let mut screen = Screen::new(640, 360);
	let mut window = screen.new_window("Test - ESC to exit").unwrap();

	let mut circle = Circle::new(50.0, 50.0, 25.0);
	let mut v = Vector2f::new(2.5, 1.5);

	window.set_target_fps(60);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		circle.position += v;
		if circle.position.x + circle.radius > screen.width as f32
			|| circle.position.x < circle.radius
		{
			v.x = -v.x;
		}
		if circle.position.y + circle.radius > screen.height as f32
			|| circle.position.y < circle.radius
		{
			v.y = -v.y;
		}

		screen.clear();
		screen.fill_circle(circle, GREEN);

		window
			.update_with_buffer(&screen, screen.width, screen.height)
			.unwrap();
	}
}
