#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{collections::BTreeMap, vec::Vec};

mod tests;

#[derive(Debug, Clone)]
pub struct Node {
	pub x: usize,
	pub y: usize,
	pub influence: Option<usize>,
}

impl Node {
	pub fn new(x: usize, y: usize, influence: Option<usize>) -> Node {
		Node { x, y, influence }
	}

	pub fn get_aabb(&self, bound: &BoundingBox) -> Option<BoundingBox> {
		match self.influence {
			Some(influence) => {
				let half_extents = influence / 2;

				bound.intersection(&BoundingBox {
					left: self.x.saturating_sub(half_extents),
					top: self.y.saturating_sub(half_extents),
					right: self.x.saturating_add(half_extents),
					bottom: self.y.saturating_add(half_extents),
				})
			}
			None => Some(bound.clone()),
		}
	}
}

#[derive(Debug, Clone)]
pub struct NoiseParams {
	pub width: usize,
	pub height: usize,
}

impl NoiseParams {
	pub fn new(width: usize, height: usize) -> NoiseParams {
		NoiseParams { width, height }
	}

	pub fn get_aabb(&self) -> BoundingBox {
		BoundingBox {
			left: 0,
			top: 0,
			right: self.width,
			bottom: self.height,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundingBox {
	pub left: usize,
	pub right: usize,
	pub top: usize,
	pub bottom: usize,
}

impl BoundingBox {
	pub fn new(x: usize, y: usize, width: usize, height: usize) -> BoundingBox {
		BoundingBox {
			left: x,
			right: x + width,
			top: y,
			bottom: y + height,
		}
	}

	pub fn contains(&self, other: &BoundingBox) -> bool {
		self.left <= other.left && self.top <= other.top && self.right >= other.right && self.bottom >= other.bottom
	}

	pub fn contains_point(&self, x: usize, y: usize) -> bool {
		x >= self.left && x < self.right && y >= self.top && y < self.bottom
	}

	pub fn intersects(&self, other: &BoundingBox) -> bool {
		let x_intersect = self.right > other.left && self.left < other.right;
		let y_intersect = self.bottom > other.top && self.top < other.bottom;

		x_intersect && y_intersect
	}

	pub fn intersection(&self, other: &BoundingBox) -> Option<BoundingBox> {
		self.intersects(other).then(|| {
			let x = self.left.max(other.left);
			let y = self.top.max(other.top);

			let x2 = self.right.min(other.right);
			let y2 = self.bottom.min(other.bottom);

			BoundingBox {
				left: x,
				top: y,
				right: x2,
				bottom: y2,
			}
		})
	}

	// subtracts the other bounding box from this bounding box
	pub fn subtract<F: FnOnce(&[BoundingBox])>(&self, rhs: &BoundingBox, callback: F) {
		// if the 2 boxes don't intersect then there is no subtraction
		if !self.intersects(rhs) {
			return callback(&[]);
		}

		let mut regions: [BoundingBox; 4] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
		let mut idx = 0;

		// chopping based algorithm
		let mut base = self.clone();

		// top
		if base.top < rhs.top {
			regions[idx] = BoundingBox { bottom: rhs.top, ..base };
			base.top = rhs.top;

			idx += 1;
		}

		// bottom
		if base.bottom > rhs.bottom {
			regions[idx] = BoundingBox { top: rhs.bottom, ..base };
			base.bottom = rhs.bottom;

			idx += 1;
		}

		// middle-left
		if base.left < rhs.left {
			regions[idx] = BoundingBox { right: rhs.left, ..base };
			base.left = rhs.left;

			idx += 1;
		}

		// middle-right
		if base.right > rhs.right {
			regions[idx] = BoundingBox { left: rhs.right, ..base };
			base.right = rhs.right;

			idx += 1;
		}

		callback(&regions[..idx]);
	}

	pub fn divide(&self, other: &BoundingBox) -> (Vec<BoundingBox>, Option<BoundingBox>, Vec<BoundingBox>) {
		// check if the 2 intersect
		unimplemented!("Given another bounding box return 3 sets of regions")
	}
}

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

fn get_regions<'a>(params: NoiseParams, nodes: &'a [Node]) -> BTreeMap<BoundingBox, Vec<usize>> {
	let mut regions: BTreeMap<BoundingBox, Vec<usize>> = BTreeMap::new();
	regions.insert(params.get_aabb(), alloc::vec![]).expect("Ok how did this happen?");

	for (idx, node) in nodes.into_iter().enumerate() {
		for (region, collect) in &mut regions {
			if let Some(influence) = node.get_aabb(region) {
				let (divided, common, subtract) = influence.divide(region);

				// iF og volume doesn't intersect with new volume then there aren't any new volumes
				if divided.is_empty() {
					debug_assert!(common.is_none(), "Common region should be None if there are no divided regions");
				}
			}
		}
	}

	regions
}
