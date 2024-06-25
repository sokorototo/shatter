#[derive(Clone)]
pub struct Stack<V, const T: usize = 32> {
	stack: [V; T],
	top: usize,
}

impl<V: core::fmt::Debug, const T: usize> core::fmt::Debug for Stack<V, T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		self.as_slice().fmt(f)
	}
}

impl<V, const T: usize> Stack<V, T> {
	pub(crate) fn new() -> Self {
		Self {
			stack: unsafe { core::mem::MaybeUninit::zeroed().assume_init() },
			top: 0,
		}
	}

	pub(crate) fn push(&mut self, value: V) {
		assert!(self.top < T, "Stack Overflow: {} >= {}", self.top, T);

		self.stack[self.top] = value;
		self.top += 1;
	}

	pub(crate) fn pop(&mut self) -> Option<V> {
		if self.top == 0 {
			None
		} else {
			self.top -= 1;
			Some(unsafe { core::ptr::read(&self.stack[self.top]) })
		}
	}

	pub(crate) fn get(&self, index: usize) -> Option<&V> {
		if index < self.top {
			Some(&self.stack[index])
		} else {
			None
		}
	}

	pub fn as_slice(&self) -> &[V] {
		&self.stack[..self.top]
	}

	pub(crate) fn len(&self) -> usize {
		self.top
	}

	pub(crate) fn swap_remove(&mut self, index: usize) -> V {
		let value = unsafe { core::ptr::read(&self.stack[index]) };
		self.top -= 1;
		self.stack.swap(index, self.top);
		value
	}
}

pub struct StackIntoIter<V, const T: usize>(Stack<V, T>);

impl<V, const T: usize> Iterator for StackIntoIter<V, T> {
	type Item = V;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

impl<V, const T: usize> IntoIterator for Stack<V, T> {
	type Item = V;
	type IntoIter = StackIntoIter<V, T>;

	fn into_iter(self) -> Self::IntoIter {
		StackIntoIter(self)
	}
}

// TODO: stack! macro
