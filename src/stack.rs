pub(crate) struct Stack<const T: usize, V> {
	stack: [V; T],
	top: usize,
}

impl<const T: usize, V> Stack<T, V> {
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

	pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut V> {
		if index < self.top {
			Some(&mut self.stack[index])
		} else {
			None
		}
	}

	pub(crate) fn as_mut_slice(&mut self) -> &mut [V] {
		&mut self.stack[..self.top]
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

pub(crate) struct StackIntoIter<const T: usize, V>(Stack<T, V>);

impl<const T: usize, V> Iterator for StackIntoIter<T, V> {
	type Item = V;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

impl<const T: usize, V> IntoIterator for Stack<T, V> {
	type Item = V;
	type IntoIter = StackIntoIter<T, V>;

	fn into_iter(self) -> Self::IntoIter {
		StackIntoIter(self)
	}
}
