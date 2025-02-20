use alloc::{rc::Rc, vec::Vec};
use core::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
};

/// Append only Reference Counted Vector with optimizations relevant to `shatter`.
/// Dereferences to an ordinary Rust slice.
#[derive(Clone)]
pub struct RcVec<T> {
	count: usize,
	source: Rc<UnsafeCell<Vec<T>>>,
}

impl<T> Deref for RcVec<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		let source = unsafe { self.source.get().as_ref().unwrap_unchecked() };
		&source[..self.count]
	}
}

impl<T> DerefMut for RcVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let source = unsafe { self.source.get().as_mut().unwrap_unchecked() };
		&mut source[..self.count]
	}
}

impl<T: PartialEq> PartialEq for RcVec<T> {
	fn eq(&self, other: &Self) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<T> From<Vec<T>> for RcVec<T> {
	fn from(value: Vec<T>) -> Self {
		RcVec::new(value)
	}
}

impl<T: core::fmt::Debug> core::fmt::Debug for RcVec<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		self.as_ref().fmt(f)
	}
}

impl<T> RcVec<T> {
	pub(crate) fn new(source: Vec<T>) -> Self {
		Self {
			count: source.len(),
			source: Rc::new(UnsafeCell::new(source)),
		}
	}

	#[must_use = "The original RcVec remains untouched, the new instance should be used"]
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
}
