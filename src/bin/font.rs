use rsr::ui::{
	Result, Window, color,
	font::{DEFAULT_FONT_HEIGHT, DEFAULT_FONT_WIDTH},
};

fn main() -> Result<()> {
	let font_size = 6;
	let margin = font_size as i32;
	let width = (16 * (DEFAULT_FONT_WIDTH + 1) + 1) * font_size;
	let height = (8 * (DEFAULT_FONT_HEIGHT + 1) + 1) * font_size;
	let mut text = String::new();
	for i in 0..128 {
		let ch = i as u8;
		if ch == b'\n' {
			text.push(' ');
		} else {
			text.push(ch as char);
		}
		if i % 16 == 15 {
			text.push('\n');
		}
	}
	let mut window = Window::new("Default Font", width, height)?;

	while window.is_open() {
		window.fill(color::WHITE);
		window.draw_text(&text, margin, margin, font_size, color::BLACK);
		window.update()?;
	}

	Ok(())
}
