#![no_std]

extern crate alloc;
pub use aabb::BoundingBox;
use alloc::vec::Vec;

mod aabb;
mod rc_vec;
pub use rc_vec::RcVec;

#[cfg(test)]
mod tests;

/// A `Node` is a point in 2D space with an optional area of influence, represented as width and height half extents.
/// A `Node` is a pending allocation into a [`BoundingBox`] to be used with [`get_regions`]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

/// Sorting `nodes` in descending order of Area of Influence massively reduces fragmentation and improves performance significantly
pub fn get_regions(root: &BoundingBox, nodes: &[Node]) -> Vec<(BoundingBox, RcVec<usize>)> {
	let mut partitions: Vec<(BoundingBox, rc_vec::RcVec<usize>)> = Vec::new();
	let mut pending = Vec::with_capacity(8);

	// Add each node's influence to the arena: Subdivide, Modify|Shrink, Merge
	for (node_idx, node) in nodes.iter().enumerate() {
		// does the node have influence on the root region?
		if let Some(influence) = node.get_influence(root) {
			// store pending partition
			pending.push(influence);

			// attempt to dissolve pending regions
			while !pending.is_empty() {
				// find first intersection with partitions in stack and partitions in the arena
				let mut needle = None;

				'search: for (stack_idx, p) in pending.as_slice().iter().enumerate() {
					for (idx, (partition, _)) in partitions.iter().enumerate() {
						if let Some(intersection) = partition.intersection(p) {
							needle = Some((intersection, idx, stack_idx));
							break 'search;
						}
					}
				}

				match needle {
					Some((intersection, partition_idx, pending_idx)) => {
						// add (pending - intersection) to the pending stack
						let (count, descendants) = pending.get(pending_idx).expect("Item not found in stack?").difference(&intersection);
						for new_pending in &descendants[..count] {
							pending.push(new_pending.clone());
						}

						// create new partition adding this node's influence
						let (_, influence) = partitions.get(partition_idx).expect("Arena doesn't contain specified item");

						let new_influence = influence.push(node_idx);
						partitions.push((intersection.clone(), new_influence));

						// add (partition - intersection) to the arena, remove old partition
						let (partition, influence) = partitions.get(partition_idx).expect("Arena doesn't contain specified item").clone();

						let (count, descendants) = partition.difference(&intersection);
						for new_region in &descendants[..count] {
							partitions.push((new_region.clone(), influence.clone()));
						}

						// remove old partitions from the arena and stack
						pending.swap_remove(pending_idx);
						partitions.swap_remove(partition_idx);
					}
					None => {
						// None of the pending regions intersect with any of the partition in the arena
						let influence = RcVec::new(alloc::vec![node_idx]);
						partitions.extend(pending.drain(..).map(|p| (p, influence.clone())));

						break;
					}
				}
			}
		}
	}

	partitions
}
