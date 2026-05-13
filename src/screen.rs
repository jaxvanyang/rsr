use super::{Float, shapes::*};
use minifb::{Window, WindowOptions};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
pub struct Screen {
	pub width: usize,
	pub height: usize,
	pub buffer: Vec<u32>,
}

impl Screen {
	pub fn new(width: usize, height: usize) -> Self {
		Screen {
			width,
			height,
			buffer: vec![0; width * height],
		}
	}

	pub fn clear(&mut self) {
		self.buffer.fill(0);
	}

	pub fn fill_rect(&mut self, rect: Rectangle, color: u32) {
		let start_x = rect.position.x as usize;
		let end_x = ((rect.position.x + rect.width) as usize).min(self.width);
		let start_y = rect.position.y as usize;
		let end_y = ((rect.position.y + rect.height) as usize).min(self.height);

		for y in start_y..end_y {
			for x in start_x..end_x {
				self[(x, y)] = color;
			}
		}
	}

	// TODO: improve performance
	pub fn fill_circle(&mut self, circle: Circle, color: u32) {
		let start_x = (circle.position.x - circle.radius).ceil() as usize;
		let end_x = ((circle.position.x + circle.radius) as usize).min(self.width - 1);
		let start_y = (circle.position.y - circle.radius).ceil() as usize;
		let end_y = ((circle.position.y + circle.radius) as usize).min(self.height - 1);

		for y in start_y..=end_y {
			for x in start_x..=end_x {
				if (x as Float - circle.position.x).hypot(y as Float - circle.position.y)
					<= circle.radius
				{
					self[(x, y)] = color;
				}
			}
		}
	}

	pub fn new_window(&self, name: &str) -> Result<Window, minifb::Error> {
		Window::new(name, self.width, self.height, WindowOptions::default())
	}

	pub fn update_window(&self, window: &mut Window) -> Result<(), minifb::Error> {
		window.update_with_buffer(&self.buffer, self.width, self.height)
	}
}

impl Deref for Screen {
	type Target = Vec<u32>;

	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}

impl DerefMut for Screen {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.buffer
	}
}

impl Index<(usize, usize)> for Screen {
	type Output = u32;

	fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
		&self.buffer[y * self.width + x]
	}
}

impl IndexMut<(usize, usize)> for Screen {
	fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
		&mut self.buffer[y * self.width + x]
	}
}
