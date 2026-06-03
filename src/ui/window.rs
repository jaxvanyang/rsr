use super::shapes::*;
use crate::{
	Float,
	pbrt::math::{round_to_left, round_to_right},
};
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

	/// Draw a pixel according to its top-left corner coordinates.
	pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
		if x < self.width && y < self.height {
			self[(x, y)] = color;
		}
	}

	/// Pixels whose center is in the rectangle are filled.
	pub fn fill_rect(&mut self, rect: Rectangle, color: u32) {
		let x_begin = round_to_left(rect.x()).max(0.) as usize;
		let y_begin = round_to_left(rect.y()).max(0.) as usize;
		let x_end = round_to_right(rect.x() + rect.w).min(self.width as Float) as usize;
		let y_end = round_to_right(rect.y() + rect.h).min(self.height as Float) as usize;

		for y in y_begin..y_end {
			for x in x_begin..x_end {
				self[(x, y)] = color;
			}
		}
	}

	// TODO: improve performance
	/// Pixels whose center is in the circle are filled.
	pub fn fill_circle(&mut self, circle: Circle, color: u32) {
		let x_begin = round_to_left(circle.x() - circle.r).max(0.) as usize;
		let y_begin = round_to_left(circle.y() - circle.r).max(0.) as usize;
		let x_end = round_to_right(circle.x() + circle.r).min(self.width as Float) as usize;
		let y_end = round_to_right(circle.y() + circle.r).min(self.height as Float) as usize;
		let r2 = circle.r.powi(2);

		for y in y_begin..y_end {
			for x in x_begin..x_end {
				let dx = x as Float + 0.5 - circle.x();
				let dy = y as Float + 0.5 - circle.y();
				if dx.powi(2) + dy.powi(2) <= r2 {
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
