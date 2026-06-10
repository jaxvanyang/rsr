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

pub trait Color {
	fn from_rgb(r: u8, g: u8, b: u8) -> Self;
	fn r(self) -> u8;
	fn g(self) -> u8;
	fn b(self) -> u8;
}

impl Color for u32 {
	fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
		let (r, g, b) = (r as u32, g as u32, b as u32);
		(r << 16) | (g << 8) | b
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
