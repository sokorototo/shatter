#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use aabb::BoundingBox;
use alloc::vec::Vec;

mod aabb;
mod stack;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Node {
	pub x: isize,
	pub y: isize,
	pub half_extent: Option<isize>,
}

impl Node {
	pub fn new(x: isize, y: isize, half_extent: Option<isize>) -> Node {
		Node { x, y, half_extent }
	}

	// creates a Bounding box based on the node's influence
	pub fn get_influence(&self, bound: &aabb::BoundingBox) -> Option<aabb::BoundingBox> {
		match self.half_extent {
			Some(half_extent) => bound.intersection(&aabb::BoundingBox {
				left: self.x.saturating_sub(half_extent),
				top: self.y.saturating_sub(half_extent),
				right: self.x.saturating_add(half_extent),
				bottom: self.y.saturating_add(half_extent),
			}),
			None => Some(bound.clone()),
		}
	}
}

pub fn get_regions<'a>(root: &BoundingBox, nodes: &'a [Node]) -> Vec<(aabb::BoundingBox, Vec<usize>)> {
	let mut index: Vec<(aabb::BoundingBox, Vec<usize>)> = Vec::new();

	// Add each node's influence to the index: Subdivide, Modify|Shrink, Merge
	for (node_idx, node) in nodes.into_iter().enumerate() {
		// does the node have influence on the root region?
		if let Some(influence) = node.get_influence(&root) {
			#[cfg(feature = "std")]
			{
				println!("[NODE & INFLUENCE]\n{:?}\n{:?}\n", node, influence);
			}

			// store pending regions
			let mut stack = stack::Stack::<32, aabb::BoundingBox>::new();
			stack.push(influence);

			'shrink: loop {
				let mut needle = None;

				'search: for (stack_idx, pending) in stack.as_slice().into_iter().enumerate() {
					for (idx, (region, _)) in index.iter().enumerate() {
						if let Some(intersection) = region.intersection(pending) {
							needle = Some((intersection, idx, stack_idx));
							break 'search;
						}
					}
				}

				#[cfg(feature = "std")]
				{
					if let Some((intersection, region_idx, pending_idx)) = needle.as_ref() {
						let item = index.get(*region_idx).unwrap();
						let pending = stack.get(*pending_idx).unwrap();

						println!(
							"[CONTACT]\nIntersection: {:?}\nFound Region: {:?}\nPending Stack Region: {:?}\n",
							intersection, item, pending
						);
					} else {
						println!("[NO CONTACT]\n");
					}
				}

				match needle {
					Some((intersection, region_idx, pending_idx)) => {
						// add (pending - intersection) to the pending stack
						let (pending_count, found) = stack.get(pending_idx).expect("Item not found in stack?").difference(&intersection);
						for new_pending in &found[..pending_count] {
							stack.push(new_pending.clone());
						}

						// create new region adding this node's influence
						let (_, influence) = index.get(region_idx).expect("Index doesn't contain specified item");

						let mut new_influence = influence.clone();
						new_influence.push(node_idx);
						index.push((intersection.clone(), new_influence));

						// add (region - intersection) to the index, remove old region
						let (region, influence) = index.get(region_idx).expect("Index doesn't contain specified item").clone();

						let (region_count, descendants) = region.difference(&intersection);
						for new_region in &descendants[..region_count] {
							index.push((new_region.clone(), influence.clone()));
						}

						// remove old regions from the index and stack, if any new regions were added
						if pending_count > 0 {
							stack.swap_remove(pending_idx);
						}
						if region_count > 0 {
							index.swap_remove(region_idx);
						}
					}
					None => {
						// None of the pending regions intersect with any of the regions in the index
						break 'shrink;
					}
				}

				#[cfg(feature = "std")]
				{
					println!("[STACK]\n{:#?}\n", stack);
					println!("[INDEX]\n{:#?}\n", index);
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
