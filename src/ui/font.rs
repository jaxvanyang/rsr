// use as 0, 1 for bool
const O: bool = false;
const I: bool = true;

pub const DEFAULT_FONT_WIDTH: usize = 5;
pub const DEFAULT_FONT_HEIGHT: usize = 6;
pub const DEFAULT_FONT_GLYPHS: [[[bool; DEFAULT_FONT_WIDTH]; DEFAULT_FONT_HEIGHT]; 128] = {
	let mut ret = [[[true; DEFAULT_FONT_WIDTH]; DEFAULT_FONT_HEIGHT]; 128];
	// A-Z
	ret[b'A' as usize] = [
		[O, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, I, I, I, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
	];
	ret[b'B' as usize] = [
		[I, I, I, I, O],
		[I, O, O, O, I],
		[I, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, I, I, I, O],
	];
	ret[b'C' as usize] = [
		[O, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, O, O, O, I],
		[O, I, I, I, O],
	];
	ret[b'D' as usize] = [
		[I, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, I, I, I, O],
	];
	ret[b'E' as usize] = [
		[I, I, I, I, I],
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, I, I, I, O],
		[I, O, O, O, O],
		[I, I, I, I, I],
	];
	ret[b'F' as usize] = [
		[I, I, I, I, I],
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, I, I, I, O],
		[I, O, O, O, O],
		[I, O, O, O, O],
	];
	ret[b'G' as usize] = [
		[O, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, O],
		[I, O, O, I, I],
		[I, O, O, O, I],
		[O, I, I, I, O],
	];
	ret[b'H' as usize] = [
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, I, I, I, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
	];
	ret[b'I' as usize] = [
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'J' as usize] = [
		[I, I, I, I, I],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[I, O, I, O, O],
		[O, I, I, O, O],
	];
	ret[b'K' as usize] = [
		[I, O, O, I, O],
		[I, O, I, O, O],
		[I, I, O, O, O],
		[I, O, I, O, O],
		[I, O, O, I, O],
		[I, O, O, O, I],
	];
	ret[b'L' as usize] = [
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, O, O, O, O],
		[I, I, I, I, I],
	];
	ret[b'M' as usize] = [
		[I, O, O, O, I],
		[I, I, O, I, I],
		[I, O, I, O, I],
		[I, O, I, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
	];
	ret[b'N' as usize] = [
		[I, O, O, O, I],
		[I, I, O, O, I],
		[I, O, I, O, I],
		[I, O, O, I, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
	];
	ret[b'O' as usize] = [
		[O, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[O, I, I, I, I],
	];
	ret[b'P' as usize] = [
		[I, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, I, I, I, O],
		[I, O, O, O, O],
		[I, O, O, O, O],
	];
	ret[b'Q' as usize] = [
		[O, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, I, O],
		[O, I, I, O, I],
	];
	ret[b'R' as usize] = [
		[I, I, I, I, O],
		[I, O, O, O, I],
		[I, O, O, I, O],
		[I, I, I, O, O],
		[I, O, O, I, O],
		[I, O, O, O, I],
	];
	ret[b'S' as usize] = [
		[O, I, I, I, O],
		[I, O, O, O, I],
		[I, I, O, O, O],
		[O, O, I, I, O],
		[I, O, O, O, I],
		[O, I, I, I, O],
	];
	ret[b'T' as usize] = [
		[I, I, I, I, I],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'U' as usize] = [
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, O, O, I],
		[O, I, I, I, O],
	];
	ret[b'V' as usize] = [
		[I, O, O, O, I],
		[I, O, O, O, I],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'W' as usize] = [
		[I, O, O, O, I],
		[I, O, O, O, I],
		[I, O, I, O, I],
		[I, O, I, O, I],
		[I, I, O, I, I],
		[I, O, O, O, I],
	];
	ret[b'X' as usize] = [
		[I, O, O, O, I],
		[O, I, O, I, O],
		[O, O, I, O, O],
		[O, I, O, I, O],
		[I, O, O, O, I],
		[I, O, O, O, I],
	];
	ret[b'Y' as usize] = [
		[I, O, O, O, I],
		[O, I, O, I, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'Z' as usize] = [
		[I, I, I, I, I],
		[O, O, O, O, I],
		[O, O, O, I, O],
		[O, O, I, O, O],
		[O, I, O, O, O],
		[I, I, I, I, I],
	];
	// a-z
	ret[b'a' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, I, O, O],
		[O, I, O, I, O],
		[O, I, I, I, I],
	];
	ret[b'b' as usize] = [
		[O, O, O, O, O],
		[O, I, O, O, O],
		[O, I, O, O, O],
		[O, I, I, O, O],
		[O, I, O, I, O],
		[O, I, I, O, O],
	];
	ret[b'c' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, I, I, O],
		[O, I, O, O, O],
		[O, I, I, I, O],
	];
	ret[b'd' as usize] = [
		[O, O, O, O, O],
		[O, O, O, I, O],
		[O, O, O, I, O],
		[O, O, I, I, O],
		[O, I, O, I, O],
		[O, O, I, I, O],
	];
	ret[b'e' as usize] = [
		[O, O, O, O, O],
		[O, O, I, I, O],
		[O, I, O, O, O],
		[O, I, I, I, O],
		[O, I, O, O, O],
		[O, O, I, I, O],
	];
	ret[b'f' as usize] = [
		[O, O, O, O, O],
		[O, O, I, I, O],
		[O, O, I, O, O],
		[O, I, I, I, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'g' as usize] = [
		[O, O, O, O, O],
		[O, I, I, I, O],
		[O, I, O, I, O],
		[O, I, I, I, O],
		[O, O, O, I, O],
		[O, O, I, I, O],
	];
	ret[b'h' as usize] = [
		[O, O, O, O, O],
		[O, I, O, O, O],
		[O, I, O, O, O],
		[O, I, I, I, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
	];
	ret[b'i' as usize] = [
		[O, O, O, O, O],
		[O, O, I, O, O],
		[O, O, O, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'j' as usize] = [
		[O, O, O, O, O],
		[O, I, I, I, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, I, I, O, O],
	];
	ret[b'k' as usize] = [
		[O, O, O, O, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, I, O, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
	];
	ret[b'l' as usize] = [
		[O, O, O, O, O],
		[O, I, O, O, O],
		[O, I, O, O, O],
		[O, I, O, O, O],
		[O, I, O, I, O],
		[O, O, I, O, O],
	];
	ret[b'm' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, O, I, O],
		[I, O, I, O, I],
		[I, O, I, O, I],
		[I, O, O, O, I],
	];
	ret[b'n' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, O, O, I],
		[O, I, I, O, I],
		[O, I, O, I, I],
		[O, I, O, O, I],
	];
	ret[b'o' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, I, I, O],
		[O, I, O, I, O],
		[O, I, I, I, O],
	];
	ret[b'p' as usize] = [
		[O, O, O, O, O],
		[O, I, I, O, O],
		[O, I, O, I, O],
		[O, I, I, O, O],
		[O, I, O, O, O],
		[O, I, O, O, O],
	];
	ret[b'q' as usize] = [
		[O, O, O, O, O],
		[O, O, I, I, O],
		[O, I, O, I, O],
		[O, O, I, I, O],
		[O, O, O, I, O],
		[O, O, O, I, O],
	];
	ret[b'r' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, I, O, O],
		[O, O, I, I, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b's' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, I, I, O],
		[O, O, I, O, O],
		[O, I, I, O, O],
	];
	ret[b't' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, I, O, O],
		[O, I, I, I, O],
		[O, O, I, O, O],
		[O, O, I, I, O],
	];
	ret[b'u' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, I, I, O],
	];
	ret[b'v' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, O, I, O, O],
	];
	ret[b'w' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[I, O, I, O, I],
		[I, O, I, O, I],
		[O, I, O, I, O],
	];
	ret[b'x' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, O, I, O],
		[O, O, I, O, O],
		[O, I, O, I, O],
	];
	ret[b'y' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
	];
	ret[b'z' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, I, I, I, I],
		[O, O, O, I, O],
		[O, O, I, O, O],
		[O, I, I, I, I],
	];
	// 0-9
	ret[b'0' as usize] = [
		[O, I, I, I, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, I, I, O],
	];
	ret[b'1' as usize] = [
		[O, O, I, O, O],
		[O, I, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, O, I, O, O],
		[O, I, I, I, O],
	];
	ret[b'2' as usize] = [
		[O, I, I, I, O],
		[O, O, O, I, O],
		[O, I, I, I, O],
		[O, I, O, O, O],
		[O, I, O, O, O],
		[O, I, I, I, O],
	];
	ret[b'3' as usize] = [
		[O, I, I, I, O],
		[O, O, O, I, O],
		[O, I, I, I, O],
		[O, O, O, I, O],
		[O, O, O, I, O],
		[O, I, I, I, O],
	];
	ret[b'4' as usize] = [
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, O, I, O],
		[O, I, I, I, I],
		[O, O, O, I, O],
		[O, O, O, I, O],
	];
	ret[b'5' as usize] = [
		[O, I, I, I, I],
		[O, I, O, O, O],
		[O, I, I, I, I],
		[O, O, O, O, I],
		[O, O, O, O, I],
		[O, I, I, I, I],
	];
	ret[b'6' as usize] = [
		[O, I, I, I, I],
		[O, I, O, O, O],
		[O, I, I, I, I],
		[O, I, O, O, I],
		[O, I, O, O, I],
		[O, I, I, I, I],
	];
	ret[b'7' as usize] = [
		[O, I, I, I, I],
		[O, O, O, O, I],
		[O, O, O, O, I],
		[O, O, O, O, I],
		[O, O, O, O, I],
		[O, O, O, O, I],
	];
	ret[b'8' as usize] = [
		[O, I, I, I, I],
		[O, I, O, O, I],
		[O, I, I, I, I],
		[O, I, O, O, I],
		[O, I, O, O, I],
		[O, I, I, I, I],
	];
	ret[b'9' as usize] = [
		[O, I, I, I, I],
		[O, I, O, O, I],
		[O, I, I, I, I],
		[O, O, O, O, I],
		[O, O, O, O, I],
		[O, I, I, I, I],
	];
	// others
	let empty = [[false; DEFAULT_FONT_WIDTH]; DEFAULT_FONT_HEIGHT];
	ret[b'\n' as usize] = empty;
	ret[b' ' as usize] = empty;
	ret[b':' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, I, O, O],
		[O, O, O, O, O],
		[O, O, I, O, O],
		[O, O, O, O, O],
	];
	ret[b'.' as usize] = [
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, O, O, O],
		[O, O, I, O, O],
	];

	ret
};
pub const DEFAULT_FONT: Font =
	Font::new(DEFAULT_FONT_WIDTH, DEFAULT_FONT_HEIGHT, flatten(&DEFAULT_FONT_GLYPHS));

const fn flatten(a: &[[[bool; DEFAULT_FONT_WIDTH]; DEFAULT_FONT_HEIGHT]; 128]) -> &[bool] {
	let len = 128 * DEFAULT_FONT_WIDTH * DEFAULT_FONT_HEIGHT;
	let ptr = a.as_ptr() as *const bool;
	unsafe { std::slice::from_raw_parts(ptr, len) }
}

#[derive(Debug)]
pub struct Font<'a> {
	pub width: usize,
	pub height: usize,
	pub glyphs: &'a [bool],
}

impl<'a> Font<'a> {
	pub const fn new(width: usize, height: usize, glyphs: &'a [bool]) -> Self {
		Self { width, height, glyphs }
	}

	pub fn get_pixel(&self, char: u8, x: usize, y: usize) -> bool {
		let idx = char as usize * self.width * self.height + y * self.width + x;
		self.glyphs[idx]
	}
}
