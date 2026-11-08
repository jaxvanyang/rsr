use minifb::{Key, KeyRepeat};
use rsr::ui::{
	Canvas, Result, Window,
	color::*,
	font::{DEFAULT_FONT_HEIGHT, DEFAULT_FONT_WIDTH},
};

fn main() -> Result<()> {
	let manim = Palette::new(
		"Manim",
		None,
		6,
		vec![
			("BLUE", manim::BLUE),
			("BLUE_A", manim::BLUE_A),
			("BLUE_B", manim::BLUE_B),
			("BLUE_C", manim::BLUE_C),
			("BLUE_D", manim::BLUE_D),
			("BLUE_E", manim::BLUE_E),
			("GOLD", manim::GOLD),
			("GOLD_A", manim::GOLD_A),
			("GOLD_B", manim::GOLD_B),
			("GOLD_C", manim::GOLD_C),
			("GOLD_D", manim::GOLD_D),
			("GOLD_E", manim::GOLD_E),
			("GRAY", manim::GRAY),
			("GRAY_A", manim::GRAY_A),
			("GRAY_B", manim::GRAY_B),
			("GRAY_C", manim::GRAY_C),
			("GRAY_D", manim::GRAY_D),
			("GRAY_E", manim::GRAY_E),
			("GREEN", manim::GREEN),
			("GREEN_A", manim::GREEN_A),
			("GREEN_B", manim::GREEN_B),
			("GREEN_C", manim::GREEN_C),
			("GREEN_D", manim::GREEN_D),
			("GREEN_E", manim::GREEN_E),
			("MAROON", manim::MAROON),
			("MAROON_A", manim::MAROON_A),
			("MAROON_B", manim::MAROON_B),
			("MAROON_C", manim::MAROON_C),
			("MAROON_D", manim::MAROON_D),
			("MAROON_E", manim::MAROON_E),
			("PURPLE", manim::PURPLE),
			("PURPLE_A", manim::PURPLE_A),
			("PURPLE_B", manim::PURPLE_B),
			("PURPLE_C", manim::PURPLE_C),
			("PURPLE_D", manim::PURPLE_D),
			("PURPLE_E", manim::PURPLE_E),
			("RED", manim::RED),
			("RED_A", manim::RED_A),
			("RED_B", manim::RED_B),
			("RED_C", manim::RED_C),
			("RED_D", manim::RED_D),
			("RED_E", manim::RED_E),
			("TEAL", manim::TEAL),
			("TEAL_A", manim::TEAL_A),
			("TEAL_B", manim::TEAL_B),
			("TEAL_C", manim::TEAL_C),
			("TEAL_D", manim::TEAL_D),
			("TEAL_E", manim::TEAL_E),
			("YELLOW", manim::YELLOW),
			("YELLOW_A", manim::YELLOW_A),
			("YELLOW_B", manim::YELLOW_B),
			("YELLOW_C", manim::YELLOW_C),
			("YELLOW_D", manim::YELLOW_D),
			("YELLOW_E", manim::YELLOW_E),
			("ORANGE", manim::ORANGE),
			("PINK", manim::PINK),
			("GRAY_BROWN", manim::GRAY_BROWN),
			("PURE_BLUE", manim::PURE_BLUE),
			("PURE_GREEN", manim::PURE_GREEN),
			("PURE_RED", manim::PURE_RED),
			("WHITE", manim::WHITE),
			("LIGHTER_GRAY", manim::LIGHTER_GRAY),
			("LIGHT_BROWN", manim::LIGHT_BROWN),
			("LIGHT_GRAY", manim::LIGHT_GRAY),
			("LIGHT_PINK", manim::LIGHT_PINK),
			("", 0),
			("BLACK", manim::BLACK),
			("DARKER_GRAY", manim::DARKER_GRAY),
			("DARK_BLUE", manim::DARK_BLUE),
			("DARK_BROWN", manim::DARK_BROWN),
			("DARK_GRAY", manim::DARK_GRAY),
			("", 0),
			("LOGO_BLACK", manim::LOGO_BLACK),
			("LOGO_BLUE", manim::LOGO_BLUE),
			("LOGO_GREEN", manim::LOGO_GREEN),
			("LOGO_RED", manim::LOGO_RED),
			("LOGO_WHITE", manim::LOGO_WHITE),
		],
	);
	let raylib = Palette::new(
		"Raylib",
		None,
		7,
		vec![
			("DARKGRAY", raylib::DARKGRAY),
			("MAROON", raylib::MAROON),
			("ORANGE", raylib::ORANGE),
			("DARKGREEN", raylib::DARKGREEN),
			("DARKBLUE", raylib::DARKBLUE),
			("DARKPURPLE", raylib::DARKPURPLE),
			("DARKBROWN", raylib::DARKBROWN),
			("GRAY", raylib::GRAY),
			("RED", raylib::RED),
			("GOLD", raylib::GOLD),
			("LIME", raylib::LIME),
			("BLUE", raylib::BLUE),
			("VIOLET", raylib::VIOLET),
			("BROWN", raylib::BROWN),
			("LIGHTGRAY", raylib::LIGHTGRAY),
			("PINK", raylib::PINK),
			("YELLOW", raylib::YELLOW),
			("GREEN", raylib::GREEN),
			("SKYBLUE", raylib::SKYBLUE),
			("PURPLE", raylib::PURPLE),
			("BEIGE", raylib::BEIGE),
			("WHITE", raylib::WHITE),
			("RAYWHITE", raylib::RAYWHITE),
			("MAGENTA", raylib::MAGENTA),
			("BLACK", raylib::BLACK),
			("BLANK", raylib::BLANK),
		],
	);
	let pico8 = Palette::new(
		"PICO 8",
		Some(16),
		2,
		vec![
			("BLACK", pico8::BLACK),
			("STORM", pico8::STORM),
			("WINE", pico8::WINE),
			("MOSS", pico8::MOSS),
			("TAN", pico8::TAN),
			("SLATE", pico8::SLATE),
			("SILVER", pico8::SILVER),
			("WHITE", pico8::WHITE),
			("EMBER", pico8::EMBER),
			("ORANGE", pico8::ORANGE),
			("LEMON", pico8::LEMON),
			("LIME", pico8::LIME),
			("SKY", pico8::SKY),
			("DUSK", pico8::DUSK),
			("PINK", pico8::PINK),
			("PEACH", pico8::PEACH),
			("COCOR", pico8::COCOR),
			("MIDNIGHT", pico8::MIDNIGHT),
			("PORT", pico8::PORT),
			("SEA", pico8::SEA),
			("LEATHER", pico8::LEATHER),
			("CHARCOAL", pico8::CHARCOAL),
			("OLIVE", pico8::OLIVE),
			("SAND", pico8::SAND),
			("CRIMSON", pico8::CRIMSON),
			("AMBER", pico8::AMBER),
			("TEA", pico8::TEA),
			("JADE", pico8::JADE),
			("DENIM", pico8::DENIM),
			("AUBERGINE", pico8::AUBERGINE),
			("SALMON", pico8::SALMON),
			("CORAL", pico8::CORAL),
		],
	);
	let tokyonight = Palette::new(
		"Tokyo Night",
		Some(8),
		3,
		vec![
			("BLACK", tokyonight::BLACK),
			("RED", tokyonight::RED),
			("GREEN", tokyonight::GREEN),
			("YELLOW", tokyonight::YELLOW),
			("BLUE", tokyonight::BLUE),
			("MAGENTA", tokyonight::MAGENTA),
			("CYAN", tokyonight::CYAN),
			("WHITE", tokyonight::WHITE),
			("BRIGHT_BLACK", tokyonight::BRIGHT_BLACK),
			("BRIGHT_RED", tokyonight::BRIGHT_RED),
			("BRIGHT_GREEN", tokyonight::BRIGHT_GREEN),
			("BRIGHT_YELLOW", tokyonight::BRIGHT_YELLOW),
			("BRIGHT_BLUE", tokyonight::BRIGHT_BLUE),
			("BRIGHT_MAGENTA", tokyonight::BRIGHT_MAGENTA),
			("BRIGHT_CYAN", tokyonight::BRIGHT_CYAN),
			("BRIGHT_WHITE", tokyonight::BRIGHT_WHITE),
			("BACKGROUND", tokyonight::BACKGROUND),
			("FOREGROUND", tokyonight::FOREGROUND),
		],
	);
	let palettes = [manim, raylib, pico8, tokyonight];
	let mut selected = 0;
	let mut window = Window::new(&format!("Palette - {}", palettes[selected].name), 1000, 750)?;

	while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
		if window.is_key_pressed(Key::Left, KeyRepeat::No) {
			selected = (selected + palettes.len() - 1) % palettes.len();
		} else if window.is_key_pressed(Key::Right, KeyRepeat::No)
			|| window.is_key_pressed(Key::Space, KeyRepeat::No)
		{
			selected = (selected + 1) % palettes.len();
		}

		window.set_title(&format!("Palette - {}", palettes[selected].name));
		window.clear();

		let cols = palettes[selected].cols;
		let rows = palettes[selected].get_rows();
		let width = window.w() / cols;
		let height = window.h() / rows;
		let font_size = palettes[selected].get_font_size(
			width,
			height,
			palettes[selected].colors.iter().map(|(name, _)| name.len()).max().unwrap_or(1),
		);

		for (i, (name, color)) in palettes[selected].colors.iter().enumerate() {
			if name.is_empty() {
				continue;
			}

			let (r, c) = palettes[selected].get_row_col(i);
			let x = (c * width) as i32;
			let y = (r * height) as i32;
			window.fill_rectangle(x, y, width, height, *color);
			window.draw_text(
				&format!("{name}\n{:06X}", color),
				x + font_size as i32,
				y + font_size as i32,
				font_size,
				get_text_color(*color),
			);
		}

		window.update()?;
	}

	Ok(())
}

struct Palette {
	pub name: &'static str,
	/// Column-first if rows is not `None`.
	pub rows: Option<usize>,
	pub cols: usize,
	pub colors: Vec<(&'static str, u32)>,
}

impl Palette {
	pub fn new(
		name: &'static str,
		rows: Option<usize>,
		cols: usize,
		colors: Vec<(&'static str, u32)>,
	) -> Self {
		Palette { name, rows, cols, colors }
	}

	pub fn get_rows(&self) -> usize {
		self.rows.unwrap_or(self.colors.len().div_ceil(self.cols))
	}

	pub fn get_row_col(&self, i: usize) -> (usize, usize) {
		if let Some(rows) = self.rows {
			(i % rows, i / rows)
		} else {
			(i / self.cols, i % self.cols)
		}
	}

	fn get_font_size(&self, width: usize, height: usize, len: usize) -> usize {
		(width / (len * (DEFAULT_FONT_WIDTH + 1))).min(height / (2 * (DEFAULT_FONT_HEIGHT + 1)))
	}
}

fn get_text_color(bg: u32) -> u32 {
	let black = 0;
	let white = 0xffffff;
	let inv = white ^ bg;
	let d = |c1: u32, c2: u32| -> i32 {
		(c1.r() as i32 - c2.r() as i32).pow(2)
			+ (c1.g() as i32 - c2.g() as i32).pow(2)
			+ (c1.b() as i32 - c2.b() as i32).pow(2)
	};
	let db = d(bg, black);
	let dw = d(bg, white);
	let di = d(bg, inv);

	if db > dw && db > di {
		black
	} else if dw > di {
		white
	} else {
		inv
	}
}
