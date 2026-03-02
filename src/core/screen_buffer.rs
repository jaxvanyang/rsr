use super::Rectangle;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct ScreenBuffer {
	pub width: usize,
	pub height: usize,
	pub buffer: Vec<u32>,
}

impl ScreenBuffer {
	pub fn new(width: usize, height: usize) -> Self {
		ScreenBuffer {
			width,
			height,
			buffer: vec![0; width * height],
		}
	}

	pub fn clear(&mut self) {
		self.buffer.fill(0);
	}

	pub fn fill_rect(&mut self, rect: Rectangle, color: u32) {
		let start_x = (rect.position.x as usize).max(0);
		let end_x = ((rect.position.x + rect.width) as usize).min(self.width);
		let start_y = (rect.position.y as usize).max(0);
		let end_y = ((rect.position.y + rect.height) as usize).min(self.height);

		for y in start_y..end_y {
			for x in start_x..end_x {
				self[(x, y)] = color;
			}
		}
	}
}

impl Deref for ScreenBuffer {
	type Target = Vec<u32>;

	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}

impl DerefMut for ScreenBuffer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.buffer
	}
}

impl Index<(usize, usize)> for ScreenBuffer {
	type Output = u32;

	fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
		&self.buffer[y * self.width + x]
	}
}

impl IndexMut<(usize, usize)> for ScreenBuffer {
	fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
		&mut self.buffer[y * self.width + x]
	}
}
