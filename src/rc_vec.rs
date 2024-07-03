use alloc::{rc::Rc, vec::Vec};
use core::cell::UnsafeCell;

/// An append only Reference Counted Vector with optimizations relevant to `shatter`.
/// Use [`RcVec::as_slice`] to get a slice view of the data.
#[derive(Clone)]
pub struct RcVec<T> {
	count: usize,
	source: Rc<UnsafeCell<Vec<T>>>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for RcVec<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		self.as_slice().fmt(f)
	}
}

impl<T> RcVec<T> {
	pub(crate) fn new(source: Vec<T>) -> Self {
		Self {
			count: source.len(),
			source: Rc::new(UnsafeCell::new(source)),
		}
	}

	#[must_use = "The original RcVec remains unused, a new modified RcVec is returned instead"]
	pub(crate) fn push(&self, item: T) -> RcVec<T> {
		unsafe {
			let source = self.source.get().as_mut().unwrap_unchecked();
			source.push(item);
		}

		RcVec {
			count: self.count + 1,
			source: self.source.clone(),
		}
	}

	/// Acquire a view of the data referenced by this [`RcVec`]
	pub fn as_slice(&self) -> &[T] {
		let source = unsafe { self.source.get().as_ref().unwrap_unchecked() };
		&source[..self.count]
	}
}
