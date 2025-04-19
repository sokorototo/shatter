use alloc::{rc::Rc, vec::Vec};
use core::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
};
use std::iter;

/// Append only Reference Counted Vector with optimizations relevant to `shatter`.
/// Dereferences to an ordinary Rust slice.
#[derive(Clone)]
pub struct RcVec<T: Clone> {
	count: usize,
	source: Rc<UnsafeCell<Vec<T>>>,
}

impl<T: Clone> Deref for RcVec<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		let source = unsafe { self.source.get().as_ref().unwrap_unchecked() };
		&source[..self.count]
	}
}

impl<T: Clone> DerefMut for RcVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let source = unsafe { self.source.get().as_mut().unwrap_unchecked() };
		&mut source[..self.count]
	}
}

impl<T: PartialEq + Clone> PartialEq for RcVec<T> {
	fn eq(&self, other: &Self) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<T: core::fmt::Debug + Clone> core::fmt::Debug for RcVec<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		self.as_ref().fmt(f)
	}
}

impl<T: Clone> RcVec<T> {
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

			// check if there are elements after self.count in reference counted vector
			if self.count < source.len() {
				// create a new source vector, to avoid overwriting data
				let new = source.as_slice()[..self.count].iter().cloned().chain(iter::once(item)).collect::<Vec<T>>();
				RcVec::new(new)
			} else {
				source.push(item);

				RcVec {
					count: self.count + 1,
					source: Rc::clone(&self.source),
				}
			}
		}
	}
}
