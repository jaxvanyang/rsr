use minifb::{Key, KeyRepeat};
use rsr::ui::{Canvas, Result, Window, color::*, font::DEFAULT_FONT_WIDTH};

fn main() -> Result<()> {
	let manim = ColorSpace::new(
		"Manim",
		vec![
			("BLACK", manim::BLACK),
			("BLUE", manim::BLUE),
			("BLUE_A", manim::BLUE_A),
			("BLUE_B", manim::BLUE_B),
			("BLUE_C", manim::BLUE_C),
			("BLUE_D", manim::BLUE_D),
			("BLUE_E", manim::BLUE_E),
			("DARKER_GRAY", manim::DARKER_GRAY),
			("DARKER_GREY", manim::DARKER_GREY),
			("DARK_BLUE", manim::DARK_BLUE),
			("DARK_BROWN", manim::DARK_BROWN),
			("DARK_GRAY", manim::DARK_GRAY),
			("DARK_GREY", manim::DARK_GREY),
			("GOLD", manim::GOLD),
			("GOLD_A", manim::GOLD_A),
			("GOLD_B", manim::GOLD_B),
			("GOLD_C", manim::GOLD_C),
			("GOLD_D", manim::GOLD_D),
			("GOLD_E", manim::GOLD_E),
			("GRAY", manim::GRAY),
			("GRAY_A", manim::GRAY_A),
			("GRAY_B", manim::GRAY_B),
			("GRAY_BROWN", manim::GRAY_BROWN),
			("GRAY_C", manim::GRAY_C),
			("GRAY_D", manim::GRAY_D),
			("GRAY_E", manim::GRAY_E),
			("GREEN", manim::GREEN),
			("GREEN_A", manim::GREEN_A),
			("GREEN_B", manim::GREEN_B),
			("GREEN_C", manim::GREEN_C),
			("GREEN_D", manim::GREEN_D),
			("GREEN_E", manim::GREEN_E),
			("GREY", manim::GREY),
			("GREY_A", manim::GREY_A),
			("GREY_B", manim::GREY_B),
			("GREY_BROWN", manim::GREY_BROWN),
			("GREY_C", manim::GREY_C),
			("GREY_D", manim::GREY_D),
			("GREY_E", manim::GREY_E),
			("LIGHTER_GRAY", manim::LIGHTER_GRAY),
			("LIGHTER_GREY", manim::LIGHTER_GREY),
			("LIGHT_BROWN", manim::LIGHT_BROWN),
			("LIGHT_GRAY", manim::LIGHT_GRAY),
			("LIGHT_GREY", manim::LIGHT_GREY),
			("LIGHT_PINK", manim::LIGHT_PINK),
			("LOGO_BLACK", manim::LOGO_BLACK),
			("LOGO_BLUE", manim::LOGO_BLUE),
			("LOGO_GREEN", manim::LOGO_GREEN),
			("LOGO_RED", manim::LOGO_RED),
			("LOGO_WHITE", manim::LOGO_WHITE),
			("MAROON", manim::MAROON),
			("MAROON_A", manim::MAROON_A),
			("MAROON_B", manim::MAROON_B),
			("MAROON_C", manim::MAROON_C),
			("MAROON_D", manim::MAROON_D),
			("MAROON_E", manim::MAROON_E),
			("ORANGE", manim::ORANGE),
			("PINK", manim::PINK),
			("PURE_BLUE", manim::PURE_BLUE),
			("PURE_GREEN", manim::PURE_GREEN),
			("PURE_RED", manim::PURE_RED),
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
			("WHITE", manim::WHITE),
			("YELLOW", manim::YELLOW),
			("YELLOW_A", manim::YELLOW_A),
			("YELLOW_B", manim::YELLOW_B),
			("YELLOW_C", manim::YELLOW_C),
			("YELLOW_D", manim::YELLOW_D),
			("YELLOW_E", manim::YELLOW_E),
		],
	);
	let raylib = ColorSpace::new(
		"Raylib",
		vec![
			("LIGHTGRAY", raylib::LIGHTGRAY),
			("GRAY", raylib::GRAY),
			("DARKGRAY", raylib::DARKGRAY),
			("YELLOW", raylib::YELLOW),
			("GOLD", raylib::GOLD),
			("ORANGE", raylib::ORANGE),
			("PINK", raylib::PINK),
			("RED", raylib::RED),
			("MAROON", raylib::MAROON),
			("GREEN", raylib::GREEN),
			("LIME", raylib::LIME),
			("DARKGREEN", raylib::DARKGREEN),
			("SKYBLUE", raylib::SKYBLUE),
			("BLUE", raylib::BLUE),
			("DARKBLUE", raylib::DARKBLUE),
			("PURPLE", raylib::PURPLE),
			("VIOLET", raylib::VIOLET),
			("DARKPURPLE", raylib::DARKPURPLE),
			("BEIGE", raylib::BEIGE),
			("BROWN", raylib::BROWN),
			("DARKBROWN", raylib::DARKBROWN),
			("WHITE", raylib::WHITE),
			("BLACK", raylib::BLACK),
			("BLANK", raylib::BLANK),
			("MAGENTA", raylib::MAGENTA),
			("RAYWHITE", raylib::RAYWHITE),
		],
	);
	let pico8 = ColorSpace::new(
		"PICO 8",
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
	let tokyonight = ColorSpace::new(
		"Tokyo Night",
		vec![
			("BACKGROUND", tokyonight::BACKGROUND),
			("FOREGROUND", tokyonight::FOREGROUND),
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
		],
	);
	let colorspaces = [manim, raylib, pico8, tokyonight];
	let mut selected = 0;
	let mut window = Window::new(&format!("Palette - {}", colorspaces[selected].name), 1000, 750)?;

	while window.is_open() {
		if window.is_key_pressed(Key::Space, KeyRepeat::No) {
			selected = (selected + 1) % colorspaces.len();
		}

		window.set_title(&format!("Palette - {}", colorspaces[selected].name));
		window.clear();

		let k = (colorspaces[selected].colors.len() as f32).sqrt().ceil() as usize;
		let width = window.w() / k;
		let height = window.h() / k;

		for (i, (name, color)) in colorspaces[selected].colors.iter().enumerate() {
			let r = i / k;
			let c = i % k;
			let x = (c * width) as i32;
			let y = (r * height) as i32;
			let font_size = get_font_size(width);
			window.fill_rect(x, y, width, height, *color);
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

struct ColorSpace {
	pub name: &'static str,
	pub colors: Vec<(&'static str, u32)>,
}

impl ColorSpace {
	pub fn new(name: &'static str, colors: Vec<(&'static str, u32)>) -> Self {
		ColorSpace { name, colors }
	}
}

fn get_font_size(width: usize) -> usize {
	width / (13 * (DEFAULT_FONT_WIDTH + 1))
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
