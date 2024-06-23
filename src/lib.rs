#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;

mod aabb;
mod stack;

#[cfg(test)]
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

pub fn get_regions<'a>(params: NoiseParams, nodes: &'a [Node]) -> Vec<(aabb::BoundingBox, Vec<usize>)> {
	let mut index: Vec<(aabb::BoundingBox, Vec<usize>)> = Vec::new();

	// Insert root region
	let root = params.get_aabb();

	// Add each node's influence to the index: Subdivide, Modify|Shrink, Merge
	for (node_idx, node) in nodes.into_iter().enumerate() {
		// does the node have influence on the root region?
		if let Some(influence) = node.get_aabb(&root) {
			// store pending regions
			let mut stack = stack::Stack::<32, aabb::BoundingBox>::new();
			stack.push(influence.clone());

			'shrink: loop {
				let mut needle = None;

				'search: for (stack_idx, pending) in stack.as_mut_slice().into_iter().enumerate() {
					for (idx, (region, _)) in index.iter().enumerate() {
						if let Some(intersection) = region.intersection(pending) {
							needle = Some((intersection, idx, stack_idx));
							break 'search;
						}
					}
				}

				match needle {
					Some((intersection, region_idx, pending_idx)) => {
						// add (pending - intersection) to the pending stack
						let (count, found) = stack.get(pending_idx).expect("Item not found in stack?").difference(&intersection);
						for new_pending in &found[..count] {
							stack.push(new_pending.clone());
						}

						// create new region adding this node's influence
						let (_, influence) = index.get(region_idx).expect("Index doesn't contain specified item");

						let mut new_influence = influence.clone();
						new_influence.push(node_idx);
						index.push((intersection.clone(), new_influence));

						// shrink the pending region, or remove if fully contained
						let (region, influence) = index.get(region_idx).expect("Index doesn't contain specified item").clone();
						let mut deferred_remove = None;

						if let Some(pending) = stack.get_mut(pending_idx) {
							if region.contains(pending) {
								deferred_remove = Some(pending_idx);
							} else {
								// add (region - pending) to the index
								let (count, found) = region.difference(pending);
								assert!(count > 0, "Region should not be fully contained by pending region");

								for new_region in &found[..count] {
									index.push((new_region.clone(), influence.clone()));
								}

								// remove old region from index
								index.swap_remove(region_idx);
							}
						}

						deferred_remove.map(|idx| stack.swap_remove(idx));
					}
					None => {
						// None of the pending regions intersect with any of the regions in the index
						break 'shrink;
					}
				}
			}

			// push all remaining regions in the stack to the index
			if stack.len() > 0 {
				let list = alloc::vec![node_idx]; // replace with Cow::Borrowed(&[idx]) if possible

				stack.into_iter().for_each(|region| {
					index.push((region, list.clone()));
				});
			}
		}
	}

	index
}
