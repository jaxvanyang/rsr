pub mod pbrt;
pub mod ui;

pub mod sys {
	use std::thread;

	pub fn nproc() -> usize {
		thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
	}
}
