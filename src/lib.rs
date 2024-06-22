#![no_std]

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
					top: self.x.saturating_sub(half_extents),
					left: self.y.saturating_sub(half_extents),
					bottom: self.x.saturating_add(half_extents),
					right: self.y.saturating_add(half_extents),
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
			top: 0,
			left: 0,
			bottom: self.width,
			right: self.height,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundingBox {
	pub top: usize,
	pub left: usize,
	pub bottom: usize,
	pub right: usize,
}

impl BoundingBox {
	pub fn new(x: usize, y: usize, width: usize, height: usize) -> BoundingBox {
		BoundingBox {
			top: x,
			left: y,
			bottom: x + width,
			right: y + height,
		}
	}

	pub fn contains(&self, other: &BoundingBox) -> bool {
		self.top <= other.top && self.left <= other.left && self.bottom >= other.bottom && self.right >= other.right
	}

	pub fn contains_point(&self, x: usize, y: usize) -> bool {
		x >= self.top && x < self.bottom && y >= self.left && y < self.right
	}

	pub fn intersection(&self, other: &BoundingBox) -> Option<BoundingBox> {
		let x_intersect = self.bottom > other.top && other.bottom > self.top;
		let y_intersect = self.right > other.left && other.right > self.left;

		(x_intersect && y_intersect).then(|| {
			let x = self.top.max(other.top);
			let y = self.left.max(other.left);

			let x2 = self.bottom.min(other.bottom);
			let y2 = self.right.min(other.right);

			BoundingBox {
				top: x,
				left: y,
				bottom: x2,
				right: y2,
			}
		})
	}

	// subtracts the other bounding box from this bounding box
	pub fn subtract(&self, rhs: &BoundingBox) -> Vec<BoundingBox> {
		let mut regions = Vec::new();

		// if other aab fully contains this aab then there are no new regions
		if rhs.contains(self) {
			return regions;
		}

		// top

		regions
	}

	pub fn divide(&self, other: &BoundingBox) -> (Vec<BoundingBox>, Option<BoundingBox>, Vec<BoundingBox>) {
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

				// is og volume doesn't intersect with new volume then there aren't any new volumes
				if divided.is_empty() {
					debug_assert!(common.is_none(), "Common region should be None if there are no divided regions");
				}
			}
		}
	}

	regions
}
