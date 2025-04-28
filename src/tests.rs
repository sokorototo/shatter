#![cfg(test)]

use super::*;
use aabb::BoundingBox;
use alloc::collections;

pub fn generate_nodes<const N: usize>(width: isize, height: isize) -> [Node; N] {
	let mut nodes: [Node; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

	for node in &mut nodes {
		*node = match simplerand::rand::<u8>() % 2 == 0 {
			// generate a square node
			true => {
				let half_extent = Some(simplerand::rand_range(25, 125));

				let x = simplerand::rand_range(5, width - 5);
				let y = simplerand::rand_range(5, height - 5);

				Node::square(x, y, half_extent)
			}
			// generate a rectangular node
			false => {
				let half_extents = {
					let hx = simplerand::rand_range(25, 125);
					let hy = simplerand::rand_range(25, 125);

					Some((hx, hy))
				};

				let x = simplerand::rand_range(5, width - 5);
				let y = simplerand::rand_range(5, height - 5);

				Node::new(x, y, half_extents)
			}
		};
	}

	nodes
}

#[test]
fn bounding_box_intersection() {
	let base = BoundingBox::new(50, 50, 100, 100);

	let normal = base.intersection(&BoundingBox::new(30, 30, 50, 50));
	let inverted = BoundingBox::new(30, 30, 50, 50).intersection(&base);
	let no_intersection = base.intersection(&BoundingBox::new(0, 0, 40, 40));
	let contained = BoundingBox::new(75, 75, 25, 25).intersection(&base);
	let contained2 = base.intersection(&BoundingBox::new(75, 75, 25, 25));

	// assert
	assert_eq!(normal, Some(BoundingBox::new(50, 50, 30, 30)));
	assert_eq!(inverted, normal);
	assert_eq!(no_intersection, None);
	assert_eq!(contained, contained2);
}

#[test]
fn bounding_box_contains() {
	let base = BoundingBox::new(50, 50, 100, 100);

	let normal = base.contains(&BoundingBox::new(60, 60, 20, 20));
	let no_contact = base.contains(&BoundingBox::new(0, 0, 40, 40));
	let intersection = base.contains(&BoundingBox::new(25, 25, 75, 75));

	assert!(normal);
	assert!(!no_contact);
	assert!(!intersection);
	assert!(base.contains(&base)); // self contains self
}

#[test]
fn bounding_box_subtract() {
	let base = BoundingBox::new(50, 50, 50, 50);

	let side = BoundingBox::new(60, 60, 20, 20);
	let (count, res) = base.subtraction(&side);
	assert_eq!(
		&res[..count],
		&[
			BoundingBox {
				left: 50,
				right: 100,
				top: 50,
				bottom: 60
			},
			BoundingBox {
				left: 50,
				right: 100,
				top: 80,
				bottom: 100
			},
			BoundingBox {
				left: 50,
				right: 60,
				top: 60,
				bottom: 80
			},
			BoundingBox {
				left: 80,
				right: 100,
				top: 60,
				bottom: 80
			}
		]
	);

	let no_intersect = BoundingBox::new(0, 0, 40, 40);
	assert_eq!(base.subtraction(&no_intersect).0, 0);

	let corner = BoundingBox::new(75, 75, 25, 25);
	let (count, res) = base.subtraction(&corner);
	assert_eq!(
		&res[..count],
		&[
			BoundingBox {
				left: 50,
				right: 100,
				top: 50,
				bottom: 75
			},
			BoundingBox {
				left: 50,
				right: 75,
				top: 75,
				bottom: 100
			}
		]
	);

	let contained = BoundingBox::new(60, 60, 20, 20);
	let (count, res) = base.subtraction(&contained);
	assert_eq!(
		&res[..count],
		[
			BoundingBox {
				left: 50,
				right: 100,
				top: 50,
				bottom: 60
			},
			BoundingBox {
				left: 50,
				right: 100,
				top: 80,
				bottom: 100
			},
			BoundingBox {
				left: 50,
				right: 60,
				top: 60,
				bottom: 80
			},
			BoundingBox {
				left: 80,
				right: 100,
				top: 60,
				bottom: 80
			}
		]
	);

	let perfectly_vertical = BoundingBox::new(50, 75, 50, 25);
	let (count, res) = base.subtraction(&perfectly_vertical);
	assert_eq!(
		&res[..count],
		&[BoundingBox {
			left: 50,
			right: 100,
			top: 50,
			bottom: 75
		}]
	);

	let fully_contained = base.clone();
	let (count, _) = base.subtraction(&fully_contained);
	assert_eq!(count, 0);
}

#[test]
fn get_node_influence() {
	let aabb = BoundingBox::new(0, 0, 200, 300);

	// define cases
	let middle = Node::square(100, 150, Some(50));
	let top_left = Node::square(0, 0, Some(50));
	let bottom_right = Node::square(200, 300, Some(50));
	let outside = Node::square(210, 310, Some(50));
	let far = Node::square(300, 400, Some(50));
	let infinite = Node::square(5000, 5000, None);

	// test cases
	assert_eq!(
		middle.intersection(&aabb),
		Some(BoundingBox {
			left: 50,
			right: 150,
			top: 100,
			bottom: 200
		})
	);
	assert_eq!(
		top_left.intersection(&aabb),
		Some(BoundingBox {
			left: 0,
			right: 50,
			top: 0,
			bottom: 50
		})
	);
	assert_eq!(
		bottom_right.intersection(&aabb),
		Some(BoundingBox {
			left: 150,
			right: 200,
			top: 250,
			bottom: 300
		})
	);
	assert_eq!(
		outside.intersection(&aabb),
		Some(BoundingBox {
			left: 160,
			right: 200,
			top: 260,
			bottom: 300
		})
	);
	assert_eq!(far.intersection(&aabb), None);
	assert_eq!(infinite.intersection(&aabb), Some(aabb));
}

#[test]
fn test_get_regions() {
	for d in 300..800 {
		let root = BoundingBox::new(0, 0, d, d);

		let nodes = generate_nodes::<25>(d, d);
		let regions = get_regions(&root, &nodes);

		let mut reverse = collections::BTreeMap::new();

		for (bounding, influence) in regions {
			for i in influence.as_ref() {
				let node = nodes.get(*i).unwrap();
				let initial = node.intersection(&root).expect("Node doesn't intersect root, but has generated region in shatter test");

				assert!(initial.contains(&bounding), "Initial Node region does not fully contain inner region");

				// add bounding to reverse map
				let _bounding = bounding.clone();
				let (_, vector) = reverse.entry(*i).or_insert_with(move || (initial, alloc::vec![_bounding]));
				vector.push(bounding.clone());
			}
		}

		// ensure sub-regions sum up to the area of the larger region
		for (_, (total, section)) in reverse {
			fn area(bb: BoundingBox) -> isize {
				(bb.right - bb.left) * (bb.bottom - bb.top)
			}

			let total_area = area(total);
			let total_section_area = section.into_iter().map(area).sum::<isize>();

			assert_eq!(total_area, total_section_area, "Total Region Area doesn't sum up to Node Area");
		}
	}
}
