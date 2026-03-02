pub const RED: u32 = 0xff0000;
pub const GREEN: u32 = 0x00ff00;
pub const BLUE: u32 = 0x0000ff;

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}
