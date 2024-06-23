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
