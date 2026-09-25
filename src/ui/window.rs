use super::{
	color,
	font::{DEFAULT_FONT, Font},
	shapes::*,
};
use crate::{
	Float,
	pbrt::{
		Vector2f, Vector2i,
		math::{round_to_left, round_to_right},
		number::Number,
	},
};
use chrono::Local;
use dirs::picture_dir;
use std::{
	fs::File,
	io::Write,
	ops::{Deref, DerefMut, Index, IndexMut},
	path::Path,
	time::Instant,
};

pub use minifb::{Result, WindowOptions};

/// Return the time in seconds elapsed from an instant and update it to now.
pub fn elapsed_with_update(t0: &mut Instant) -> Float {
	let t1 = Instant::now();
	let ret = t1.duration_since(*t0).as_secs_f64().as_float();
	*t0 = t1;

	ret
}

#[derive(Debug)]
pub struct Window {
	pub window: minifb::Window,
	pub buffer: ScreenBuffer,
	t0: Instant,
	/// delta time of the last frame in seconds
	dt: Float,
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
			window,
			buffer: ScreenBuffer::new(width, height),
			t0: Instant::now(),
			dt: 0.,
		})
	}

	pub fn update(&mut self) -> Result<()> {
		let ret = self
			.window
			.update_with_buffer(&self.buffer.buffer, self.buffer.width, self.buffer.height);
		self.dt = elapsed_with_update(&mut self.t0);
		ret
	}

	/// Return the delta time of the last frame in seconds.
	pub fn delta_time(&self) -> Float {
		self.dt
	}

	pub fn draw_fps(&mut self, x: i32, y: i32) {
		let text = format!("FPS:{:.1}", 1. / self.dt);
		self.draw_text(&text, x, y, 2, color::GREEN);
	}

	pub fn take_screenshot(&self) -> std::io::Result<()> {
		let dir = picture_dir().unwrap().join("Screenshots");
		std::fs::create_dir_all(&dir)?;

		let path = dir.join(format!("Screenshot_{}.ppm", Local::now().format("%Y-%m-%d_%H-%M-%S")));
		self.save_ppm(&path)?;
		println!("Screenshot saved to {}", path.display());

		Ok(())
	}
}

impl Deref for Window {
	type Target = minifb::Window;

	fn deref(&self) -> &Self::Target {
		&self.window
	}
}

impl DerefMut for Window {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.window
	}
}

impl Index<(usize, usize)> for Window {
	type Output = u32;

	fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
		&self.buffer[(x, y)]
	}
}

impl IndexMut<(usize, usize)> for Window {
	fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
		&mut self.buffer[(x, y)]
	}
}

impl Canvas for Window {
	fn w(&self) -> usize {
		self.buffer.w()
	}

	fn h(&self) -> usize {
		self.buffer.h()
	}

	fn fill(&mut self, color: u32) {
		self.buffer.fill(color);
	}

	fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
		self.buffer.draw_pixel(x, y, color);
	}
}

#[derive(Debug)]
pub struct ScreenBuffer {
	pub width: usize,
	pub height: usize,
	pub buffer: Vec<u32>,
}

impl ScreenBuffer {
	pub fn new(width: usize, height: usize) -> Self {
		Self { width, height, buffer: vec![0; width * height] }
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

impl Canvas for ScreenBuffer {
	fn w(&self) -> usize {
		self.width
	}

	fn h(&self) -> usize {
		self.height
	}

	fn fill(&mut self, color: u32) {
		self.buffer.fill(color);
	}

	/// Draw a pixel according to its top-left corner coordinates.
	fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
		if x < 0 || y < 0 {
			return;
		}
		let (x, y) = (x as usize, y as usize);
		if x < self.width && y < self.height {
			self[(x, y)] = color;
		}
	}
}

pub trait Canvas: Index<(usize, usize), Output = u32> + IndexMut<(usize, usize)> {
	/// Width.
	fn w(&self) -> usize;
	/// Height.
	fn h(&self) -> usize;

	fn clear(&mut self) {
		self.fill(0);
	}

	fn fill(&mut self, color: u32);

	/// Draw a pixel according to its top-left corner coordinates.
	fn draw_pixel(&mut self, x: i32, y: i32, color: u32);

	fn draw_pixel_f(&mut self, x: Float, y: Float, color: u32) {
		self.draw_pixel(x as i32, y as i32, color);
	}

	fn draw_pixel_vf(&mut self, p: Vector2f, color: u32) {
		self.draw_pixel(p.x as i32, p.y as i32, color);
	}

	fn draw_pixel_vi(&mut self, p: Vector2i, color: u32) {
		self.draw_pixel(p.x, p.y, color);
	}

	/// Draw a line from pixel `(x0, y0)` to `(x1, y1)`, use Bresenham's line algorithm.
	fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
		let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);
		let steep = (y1 - y0).abs() > (x1 - x0).abs();
		if steep {
			(x0, y0) = (y0, x0);
			(x1, y1) = (y1, x1);
		}
		if x0 > x1 {
			(x0, y0, x1, y1) = (x1, y1, x0, y0);
		}
		let dx = x1 - x0;
		let dx2 = dx * 2;
		let dy2 = (y1 - y0).abs() * 2;
		let y_step = if y0 < y1 { 1 } else { -1 };

		let mut error = dx;
		let mut y = y0;
		for x in x0..=x1 {
			if steep {
				self.draw_pixel(y, x, color);
			} else {
				self.draw_pixel(x, y, color)
			}
			error -= dy2;
			if error <= 0 {
				y += y_step;
				error += dx2;
			}
		}
	}

	fn draw_lines(&mut self, points: &[Vector2i], color: u32) {
		if points.len() < 2 {
			return;
		}

		for i in 1..points.len() {
			let (p, q) = (points[i - 1], points[i]);
			self.draw_line(p.x, p.y, q.x, q.y, color);
		}
	}

	fn draw_polygon(&mut self, points: &[Vector2i], color: u32) {
		if points.len() < 2 {
			return;
		}

		for (i, p) in points.iter().enumerate() {
			let q = points[(i + 1) % points.len()];
			self.draw_line(p.x, p.y, q.x, q.y, color);
		}
	}

	fn draw_triangle(&mut self, p0: Vector2i, p1: Vector2i, p2: Vector2i, color: u32) {
		self.draw_line(p0.x, p0.y, p1.x, p1.y, color);
		self.draw_line(p1.x, p1.y, p2.x, p2.y, color);
		self.draw_line(p2.x, p2.y, p0.x, p0.y, color);
	}

	fn draw_text(&mut self, text: &str, x: i32, y: i32, font_size: usize, color: u32) {
		self.draw_font_text(&DEFAULT_FONT, text, x, y, font_size, color);
	}

	fn draw_font_text(
		&mut self,
		font: &Font,
		text: &str,
		x: i32,
		y: i32,
		font_size: usize,
		color: u32,
	) {
		let w = ((font.width + 1) * font_size) as i32;
		let h = ((font.height + 1) * font_size) as i32;
		let mut x_start = x;
		let mut y_start = y;
		for ch in text.bytes() {
			for dy in 0..font.height {
				for dx in 0..font.width {
					if !font.get_pixel(ch, dx, dy) {
						continue;
					}
					self.fill_rect(
						x_start + (dx * font_size) as i32,
						y_start + (dy * font_size) as i32,
						font_size,
						font_size,
						color,
					);
				}
			}
			if ch == b'\n' {
				x_start = x;
				y_start += h;
			} else {
				x_start += w;
			}
		}
	}

	/// Pixels whose center is in the rectangle are filled.
	fn fill_rectangle(&mut self, rect: Rectangle, color: u32) {
		let x_begin = round_to_left(rect.x()).max(0.) as usize;
		let y_begin = round_to_left(rect.y()).max(0.) as usize;
		let x_end = round_to_right(rect.x() + rect.w).min(self.w() as Float) as usize;
		let y_end = round_to_right(rect.y() + rect.h).min(self.h() as Float) as usize;

		for y in y_begin..y_end {
			for x in x_begin..x_end {
				self[(x, y)] = color;
			}
		}
	}

	fn fill_rect(&mut self, x: i32, y: i32, w: usize, h: usize, color: u32) {
		let x_begin = Ord::max(x, 0) as usize;
		let y_begin = Ord::max(y, 0) as usize;
		let x_end = Ord::min(x + w as i32, self.w() as i32) as usize;
		let y_end = Ord::min(y + h as i32, self.h() as i32) as usize;

		for y in y_begin..y_end {
			for x in x_begin..x_end {
				self[(x, y)] = color;
			}
		}
	}

	// TODO: improve performance
	/// Pixels whose center is in the circle are filled.
	fn fill_circle(&mut self, circle: Circle, color: u32) {
		let x_begin = round_to_left(circle.x() - circle.r).max(0.) as usize;
		let y_begin = round_to_left(circle.y() - circle.r).max(0.) as usize;
		let x_end = round_to_right(circle.x() + circle.r).min(self.w() as Float) as usize;
		let y_end = round_to_right(circle.y() + circle.r).min(self.h() as Float) as usize;
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

	/// # Note
	///
	/// This might go wrong with concave polygon.
	fn fill_polygon(&mut self, points: &[Vector2i], color: u32) {
		if points.len() < 2 {
			return;
		}

		let y_min = Ord::max(points.iter().map(|p| p.y).min().unwrap(), 0);
		let y_max = points.iter().map(|p| p.y).max().unwrap();
		let y_end = Ord::min(y_max + 1, self.h() as i32);

		for y in y_min..y_end {
			let mut xs = Vec::new();

			for (i, p) in points.iter().enumerate() {
				let q = points[(i + 1) % points.len()];

				// only consider the upper end to avoid duplication
				if !((p.y <= y && y < q.y) || (q.y <= y && y < p.y)) {
					continue;
				}

				let (dx, dy) = (q.x - p.x, q.y - p.y);
				// no need to convert to float because we have to round to integer anyway
				let x = dx * (y - p.y) / dy + p.x;
				xs.push(x);
			}

			xs.sort();

			for i in 0..(xs.len() / 2) {
				let (x0, x1) = (xs[i * 2], xs[i * 2 + 1]);
				let (x0, x1) = (Ord::max(x0, 0), Ord::min(x1, self.w() as i32 - 1));
				for x in x0..=x1 {
					self[(x as usize, y as usize)] = color;
				}
			}
			if xs.len() % 2 == 1 {
				self.draw_pixel(*xs.last().unwrap(), y, color);
			}
		}
	}

	fn to_ppm(&self) -> Vec<u8> {
		let mut ppm = format!("P6\n{} {}\n255\n", self.w(), self.h()).as_bytes().to_vec();
		for y in 0..self.h() {
			for x in 0..self.w() {
				let color = self[(x, y)];
				let r = ((color >> 16) & 0xFF) as u8;
				let g = ((color >> 8) & 0xFF) as u8;
				let b = (color & 0xFF) as u8;
				ppm.push(r);
				ppm.push(g);
				ppm.push(b);
			}
		}
		ppm
	}

	fn save_ppm<P>(&self, path: P) -> std::io::Result<()>
	where
		P: AsRef<Path>,
	{
		let ppm = self.to_ppm();
		let mut file = File::create(path)?;
		file.write_all(&ppm)?;

		Ok(())
	}
}
