use super::shapes::*;
use crate::Float;
use std::ops;

pub use minifb::{Result, WindowOptions};

#[derive(Debug)]
pub struct Window {
	pub width: usize,
	pub height: usize,
	pub window: minifb::Window,
	pub buffer: Vec<u32>,
}

impl Window {
	pub fn new(name: &str, width: usize, height: usize) -> Result<Self> {
		Self::new_with_opts(name, width, height, WindowOptions::default())
	}

	pub fn new_with_opts(
		name: &str,
		width: usize,
		height: usize,
		opts: WindowOptions,
	) -> Result<Self> {
		minifb::Window::new(name, width, height, opts).map(|window| Self {
			width,
			height,
			window,
			buffer: vec![0; width * height],
		})
	}

	pub fn update(&mut self) -> Result<()> {
		self.window
			.update_with_buffer(&self.buffer, self.width, self.height)
	}

	pub fn clear(&mut self) {
		self.fill(0);
	}

	pub fn fill(&mut self, color: u32) {
		self.buffer.fill(color);
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
}

impl ops::Index<(usize, usize)> for Window {
	type Output = u32;

	fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
		&self.buffer[y * self.width + x]
	}
}

impl ops::IndexMut<(usize, usize)> for Window {
	fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
		&mut self.buffer[y * self.width + x]
	}
}

impl ops::Deref for Window {
	type Target = minifb::Window;

	fn deref(&self) -> &Self::Target {
		&self.window
	}
}

impl ops::DerefMut for Window {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.window
	}
}
