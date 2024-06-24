#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
pub use aabb::BoundingBox;
use alloc::{rc::Rc, vec::Vec};

mod aabb;
mod stack;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Node {
	pub x: isize,
	pub y: isize,
	pub half_extents: Option<(isize, isize)>,
}

impl Node {
	pub fn square(x: isize, y: isize, half_extent: Option<isize>) -> Node {
		let half_extents = half_extent.map(|h| (h, h));
		Node { x, y, half_extents }
	}

	pub fn new(x: isize, y: isize, half_extents: Option<(isize, isize)>) -> Node {
		Node { x, y, half_extents }
	}

	// creates a Bounding box based on the node's influence
	pub fn get_influence(&self, bound: &aabb::BoundingBox) -> Option<aabb::BoundingBox> {
		match self.half_extents {
			Some((x, y)) => bound.intersection(&aabb::BoundingBox {
				left: self.x.saturating_sub(x),
				top: self.y.saturating_sub(y),
				right: self.x.saturating_add(x),
				bottom: self.y.saturating_add(y),
			}),
			None => Some(bound.clone()),
		}
	}
}

pub fn get_regions<'a>(root: &BoundingBox, nodes: &'a [Node]) -> Vec<(aabb::BoundingBox, Rc<Vec<usize>>)> {
	let mut arena: Vec<(aabb::BoundingBox, Rc<Vec<usize>>)> = Vec::new();

	// Add each node's influence to the arena: Subdivide, Modify|Shrink, Merge
	for (node_idx, node) in nodes.into_iter().enumerate() {
		// does the node have influence on the root region?
		if let Some(influence) = node.get_influence(&root) {
			// store pending regions
			let mut stack = stack::Stack::<32, aabb::BoundingBox>::new();
			stack.push(influence);

			'shrink: loop {
				// if stack is empty, we're done
				if stack.len() == 0 {
					break 'shrink;
				}

				// find first intersection with regions in stack and regions in the arena
				let mut needle = None;

				'search: for (stack_idx, pending) in stack.as_slice().into_iter().enumerate() {
					for (idx, (region, _)) in arena.iter().enumerate() {
						if let Some(intersection) = region.intersection(pending) {
							needle = Some((intersection, idx, stack_idx));
							break 'search;
						}
					}
				}

				match needle {
					Some((intersection, region_idx, pending_idx)) => {
						// add (pending - intersection) to the pending stack
						let (count, descendants) = stack.get(pending_idx).expect("Item not found in stack?").difference(&intersection);
						for new_pending in &descendants[..count] {
							stack.push(new_pending.clone());
						}

						// create new region adding this node's influence
						let (_, influence) = arena.get(region_idx).expect("Arena doesn't contain specified item");

						let mut new_influence = Vec::clone(&influence);
						new_influence.push(node_idx);
						arena.push((intersection.clone(), Rc::new(new_influence)));

						// add (region - intersection) to the arena, remove old region
						let (region, influence) = arena.get(region_idx).expect("Arena doesn't contain specified item").clone();

						let (count, descendants) = region.difference(&intersection);
						for new_region in &descendants[..count] {
							arena.push((new_region.clone(), influence.clone()));
						}

						// remove old regions from the arena and stack
						stack.swap_remove(pending_idx);
						arena.swap_remove(region_idx);
					}
					None => {
						// None of the pending regions intersect with any of the regions in the arena
						break 'shrink;
					}
				}
			}

			// push all remaining regions in the stack to the arena
			if stack.len() > 0 {
				let list = Rc::new(alloc::vec![node_idx]); // replace with Cow::Borrowed(&[idx]) if possible

				stack.into_iter().for_each(|region| {
					arena.push((region, list.clone()));
				});
			}
		}
	}

	arena
}
