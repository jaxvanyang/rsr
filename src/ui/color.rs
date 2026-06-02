pub const RED: u32 = 0xff0000;
pub const GREEN: u32 = 0x00ff00;
pub const BLUE: u32 = 0x0000ff;

pub fn new_color(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}

pub trait Color {
	fn r(self) -> u8;
	fn g(self) -> u8;
	fn b(self) -> u8;
}

impl Color for u32 {
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
		let c = new_color(1, 2, 3);
		assert_eq!(c.r(), 1);
		assert_eq!(c.g(), 2);
		assert_eq!(c.b(), 3);
	}
}
