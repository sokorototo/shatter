#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{collections::BTreeMap, vec::Vec};

mod aabb;
mod stack;
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

	// creates a Bounding box based on the node's influence
	pub fn get_aabb(&self, bound: &aabb::BoundingBox) -> Option<aabb::BoundingBox> {
		match self.influence {
			Some(influence) => {
				let half_extents = influence / 2;

				bound.intersection(&aabb::BoundingBox {
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

	pub fn get_aabb(&self) -> aabb::BoundingBox {
		aabb::BoundingBox {
			left: 0,
			top: 0,
			right: self.width,
			bottom: self.height,
		}
	}
}

fn get_regions<'a>(params: NoiseParams, nodes: &'a [Node]) -> BTreeMap<aabb::BoundingBox, Vec<usize>> {
	let mut regions: BTreeMap<aabb::BoundingBox, Vec<usize>> = BTreeMap::new();

	// Insert root region
	let root = params.get_aabb();
	regions.insert(root.clone(), alloc::vec![]).expect("Ok how did this happen?");

	for (idx, node) in nodes.into_iter().enumerate() {
		let mut unallocated = alloc::vec![];

		// does the node have influence on the root region?
		if let Some(influence) = node.get_aabb(&root) {
			for (region, collect) in &mut regions {
				// unaffected regions that inherit node properties of the original
				let mut new_unaffected;
				region.difference(&influence, |d| new_unaffected = d.to_vec());

				// new region that will inherit an extra node, built from an intersection
				let new_affected = region.intersection(&influence).expect("Intersection failed");

				// new regions spawned just for this node
				let mut new_custom;
				influence.difference(region, |d| new_custom = d.to_vec());
			}
		}
	}

	regions
}
