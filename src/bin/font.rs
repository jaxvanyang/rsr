use rsr::ui::{Result, Window, color};

fn main() -> Result<()> {
	let mut window = Window::new("Default Font", 960, 360)?;
	let text = " !\"#$%&'()*+,-./\n0123456789\n:;<=>?@\nABCDEFGHIJKLMNOPQRSTUVWXYZ\n[\\]^_`\nabcdefghijklmnopqrstuvwxyz\n{|}~";
	let font_size = 6;
	let margin = font_size as i32;

	while window.is_open() {
		window.fill(color::WHITE);
		window.draw_text(text, margin, margin, font_size, color::BLACK);
		window.update()?;
	}

	Ok(())
}
