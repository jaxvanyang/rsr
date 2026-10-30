pub mod manim {
	// Manim colors: https://docs.manim.community/en/stable/reference/manim.utils.color.manim_colors.html
	pub const BLACK: u32 = 0x000000;
	pub const BLUE: u32 = 0x58C4DD;
	pub const BLUE_A: u32 = 0xC7E9F1;
	pub const BLUE_B: u32 = 0x9CDCEB;
	pub const BLUE_C: u32 = 0x58C4DD;
	pub const BLUE_D: u32 = 0x29ABCA;
	pub const BLUE_E: u32 = 0x236B8E;
	pub const DARKER_GRAY: u32 = 0x222222;
	pub const DARKER_GREY: u32 = 0x222222;
	pub const DARK_BLUE: u32 = 0x236B8E;
	pub const DARK_BROWN: u32 = 0x8B4513;
	pub const DARK_GRAY: u32 = 0x444444;
	pub const DARK_GREY: u32 = 0x444444;
	pub const GOLD: u32 = 0xF0AC5F;
	pub const GOLD_A: u32 = 0xF7C797;
	pub const GOLD_B: u32 = 0xF9B775;
	pub const GOLD_C: u32 = 0xF0AC5F;
	pub const GOLD_D: u32 = 0xE1A158;
	pub const GOLD_E: u32 = 0xC78D46;
	pub const GRAY: u32 = 0x888888;
	pub const GRAY_A: u32 = 0xDDDDDD;
	pub const GRAY_B: u32 = 0xBBBBBB;
	pub const GRAY_BROWN: u32 = 0x736357;
	pub const GRAY_C: u32 = 0x888888;
	pub const GRAY_D: u32 = 0x444444;
	pub const GRAY_E: u32 = 0x222222;
	pub const GREEN: u32 = 0x83C167;
	pub const GREEN_A: u32 = 0xC9E2AE;
	pub const GREEN_B: u32 = 0xA6CF8C;
	pub const GREEN_C: u32 = 0x83C167;
	pub const GREEN_D: u32 = 0x77B05D;
	pub const GREEN_E: u32 = 0x699C52;
	pub const GREY: u32 = 0x888888;
	pub const GREY_A: u32 = 0xDDDDDD;
	pub const GREY_B: u32 = 0xBBBBBB;
	pub const GREY_BROWN: u32 = 0x736357;
	pub const GREY_C: u32 = 0x888888;
	pub const GREY_D: u32 = 0x444444;
	pub const GREY_E: u32 = 0x222222;
	pub const LIGHTER_GRAY: u32 = 0xDDDDDD;
	pub const LIGHTER_GREY: u32 = 0xDDDDDD;
	pub const LIGHT_BROWN: u32 = 0xCD853F;
	pub const LIGHT_GRAY: u32 = 0xBBBBBB;
	pub const LIGHT_GREY: u32 = 0xBBBBBB;
	pub const LIGHT_PINK: u32 = 0xDC75CD;
	pub const LOGO_BLACK: u32 = 0x343434;
	pub const LOGO_BLUE: u32 = 0x525893;
	pub const LOGO_GREEN: u32 = 0x87C2A5;
	pub const LOGO_RED: u32 = 0xE07A5F;
	pub const LOGO_WHITE: u32 = 0xECE7E2;
	pub const MAROON: u32 = 0xC55F73;
	pub const MAROON_A: u32 = 0xECABC1;
	pub const MAROON_B: u32 = 0xEC92AB;
	pub const MAROON_C: u32 = 0xC55F73;
	pub const MAROON_D: u32 = 0xA24D61;
	pub const MAROON_E: u32 = 0x94424F;
	pub const ORANGE: u32 = 0xFF862F;
	pub const PINK: u32 = 0xD147BD;
	pub const PURE_BLUE: u32 = 0x0000FF;
	pub const PURE_GREEN: u32 = 0x00FF00;
	pub const PURE_RED: u32 = 0xFF0000;
	pub const PURPLE: u32 = 0x9A72AC;
	pub const PURPLE_A: u32 = 0xCAA3E8;
	pub const PURPLE_B: u32 = 0xB189C6;
	pub const PURPLE_C: u32 = 0x9A72AC;
	pub const PURPLE_D: u32 = 0x715582;
	pub const PURPLE_E: u32 = 0x644172;
	pub const RED: u32 = 0xFC6255;
	pub const RED_A: u32 = 0xF7A1A3;
	pub const RED_B: u32 = 0xFF8080;
	pub const RED_C: u32 = 0xFC6255;
	pub const RED_D: u32 = 0xE65A4C;
	pub const RED_E: u32 = 0xCF5044;
	pub const TEAL: u32 = 0x5CD0B3;
	pub const TEAL_A: u32 = 0xACEAD7;
	pub const TEAL_B: u32 = 0x76DDC0;
	pub const TEAL_C: u32 = 0x5CD0B3;
	pub const TEAL_D: u32 = 0x55C1A7;
	pub const TEAL_E: u32 = 0x49A88F;
	pub const WHITE: u32 = 0xFFFFFF;
	pub const YELLOW: u32 = 0xFFFF00;
	pub const YELLOW_A: u32 = 0xFFF1B6;
	pub const YELLOW_B: u32 = 0xFFEA94;
	pub const YELLOW_C: u32 = 0xFFFF00;
	pub const YELLOW_D: u32 = 0xF4D345;
	pub const YELLOW_E: u32 = 0xE8C11C;
}

pub mod raylib {
	use super::new_color;

	pub const LIGHTGRAY: u32 = new_color(200, 200, 200);
	pub const GRAY: u32 = new_color(130, 130, 130);
	pub const DARKGRAY: u32 = new_color(80, 80, 80);
	pub const YELLOW: u32 = new_color(253, 249, 0);
	pub const GOLD: u32 = new_color(255, 203, 0);
	pub const ORANGE: u32 = new_color(255, 161, 0);
	pub const PINK: u32 = new_color(255, 109, 194);
	pub const RED: u32 = new_color(230, 41, 55);
	pub const MAROON: u32 = new_color(190, 33, 55);
	pub const GREEN: u32 = new_color(0, 228, 48);
	pub const LIME: u32 = new_color(0, 158, 47);
	pub const DARKGREEN: u32 = new_color(0, 117, 44);
	pub const SKYBLUE: u32 = new_color(102, 191, 255);
	pub const BLUE: u32 = new_color(0, 121, 241);
	pub const DARKBLUE: u32 = new_color(0, 82, 172);
	pub const PURPLE: u32 = new_color(200, 122, 255);
	pub const VIOLET: u32 = new_color(135, 60, 190);
	pub const DARKPURPLE: u32 = new_color(112, 31, 126);
	pub const BEIGE: u32 = new_color(211, 176, 131);
	pub const BROWN: u32 = new_color(127, 106, 79);
	pub const DARKBROWN: u32 = new_color(76, 63, 47);

	pub const WHITE: u32 = new_color(255, 255, 255);
	pub const BLACK: u32 = new_color(0, 0, 0);
	pub const BLANK: u32 = new_color(0, 0, 0);
	pub const MAGENTA: u32 = new_color(255, 0, 255);
	pub const RAYWHITE: u32 = new_color(245, 245, 245);
}

pub mod pico8 {
	pub const BLACK: u32 = 0x000000;
	pub const STORM: u32 = 0x102B53;
	pub const WINE: u32 = 0x7E2553;
	pub const MOSS: u32 = 0x008751;
	pub const TAN: u32 = 0xAB5236;
	pub const SLATE: u32 = 0x5F574F;
	pub const SILVER: u32 = 0xC2C3C7;
	pub const WHITE: u32 = 0xFFF1E8;
	pub const EMBER: u32 = 0xFF004D;
	pub const ORANGE: u32 = 0xFFA300;
	pub const LEMON: u32 = 0xFFEC27;
	pub const LIME: u32 = 0x00E43B;
	pub const SKY: u32 = 0x29ADFF;
	pub const DUSK: u32 = 0x83769C;
	pub const PINK: u32 = 0xFF77A8;
	pub const PEACH: u32 = 0xFFCCAA;

	pub const COCOR: u32 = 0x291814;
	pub const MIDNIGHT: u32 = 0x111D35;
	pub const PORT: u32 = 0x422136;
	pub const SEA: u32 = 0x125359;
	pub const LEATHER: u32 = 0x742F29;
	pub const CHARCOAL: u32 = 0x49333E;
	pub const OLIVE: u32 = 0xA28879;
	pub const SAND: u32 = 0xF3EF7D;
	pub const CRIMSON: u32 = 0xBE1250;
	pub const AMBER: u32 = 0xFF6C24;
	pub const TEA: u32 = 0xA8E72E;
	pub const JADE: u32 = 0x00B543;
	pub const DENIM: u32 = 0x065AB5;
	pub const AUBERGINE: u32 = 0x754665;
	pub const SALMON: u32 = 0xFF6e59;
	pub const CORAL: u32 = 0xFF9D81;
}

pub mod tokyonight {
	pub const BACKGROUND: u32 = 0x1A1B26;
	pub const FOREGROUND: u32 = 0xC0CAF5;

	pub const BLACK: u32 = 0x15161E;
	pub const RED: u32 = 0xF7768E;
	pub const GREEN: u32 = 0x9ECE6A;
	pub const YELLOW: u32 = 0xE0AF68;
	pub const BLUE: u32 = 0x7AA2F7;
	pub const MAGENTA: u32 = 0xBB9AF7;
	pub const CYAN: u32 = 0x7DCFFF;
	pub const WHITE: u32 = 0xA9B1D6;

	pub const BRIGHT_BLACK: u32 = 0x414868;
	pub const BRIGHT_RED: u32 = 0xFF899D;
	pub const BRIGHT_GREEN: u32 = 0x9FE044;
	pub const BRIGHT_YELLOW: u32 = 0xFABA4A;
	pub const BRIGHT_BLUE: u32 = 0x8DB0FF;
	pub const BRIGHT_MAGENTA: u32 = 0xC7A9FF;
	pub const BRIGHT_CYAN: u32 = 0xA4DAFF;
	pub const BRIGHT_WHITE: u32 = 0xC0CAF5;
}

pub trait Color {
	fn from_rgb(r: u8, g: u8, b: u8) -> Self;
	fn r(self) -> u8;
	fn g(self) -> u8;
	fn b(self) -> u8;
}

impl Color for u32 {
	fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
		new_color(r, g, b)
	}

	fn r(self) -> u8 {
		((self & 0x00ff0000) >> 16) as u8
	}
	fn g(self) -> u8 {
		((self & 0x0000ff00) >> 8) as u8
	}
	fn b(self) -> u8 {
		self as u8
	}
}

pub const fn new_color(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_color() {
		let c = u32::from_rgb(1, 2, 3);
		assert_eq!(c.r(), 1);
		assert_eq!(c.g(), 2);
		assert_eq!(c.b(), 3);
	}
}
