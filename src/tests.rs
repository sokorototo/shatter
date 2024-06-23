use super::*;
use aabb::BoundingBox;
use alloc::vec;

#[test]
fn bounding_box_intersection() {
	let base = BoundingBox::new(50, 50, 100, 100);

	let normal = base.intersection(&BoundingBox::new(30, 30, 50, 50));
	let no_intersection = base.intersection(&BoundingBox::new(0, 0, 40, 40));
	let contained = BoundingBox::new(75, 75, 25, 25).intersection(&base);
	let contained2 = base.intersection(&BoundingBox::new(75, 75, 25, 25));

	// assert
	assert_eq!(normal, Some(BoundingBox::new(50, 50, 30, 30)));
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
	let (count, res) = base.difference(&side);
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
	assert_eq!(base.difference(&no_intersect).0, 0);

	let corner = BoundingBox::new(75, 75, 25, 25);
	let (count, res) = base.difference(&corner);
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
	let (count, res) = base.difference(&contained);
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
	let (count, res) = base.difference(&perfectly_vertical);
	assert_eq!(
		&res[..count],
		&[BoundingBox {
			left: 50,
			right: 100,
			top: 50,
			bottom: 75
		}]
	);
}

#[test]
fn get_node_aabb() {
	let params = NoiseParams::new(200, 300);
	let aabb = params.get_aabb();

	// define cases
	let middle = Node::new(100, 150, Some(50));
	let top_left = Node::new(0, 0, Some(50));
	let bottom_right = Node::new(200, 300, Some(50));
	let outside = Node::new(210, 310, Some(50));
	let far = Node::new(300, 400, Some(50));
	let infinite = Node::new(5000, 5000, None);

	// test cases
	assert_eq!(middle.get_aabb(&aabb), Some(BoundingBox::new(75, 125, 50, 50)));
	assert_eq!(top_left.get_aabb(&aabb), Some(BoundingBox::new(0, 0, 25, 25)));
	assert_eq!(bottom_right.get_aabb(&aabb), Some(BoundingBox::new(175, 275, 25, 25)));
	assert_eq!(outside.get_aabb(&aabb), Some(BoundingBox::new(185, 285, 15, 15)));
	assert_eq!(far.get_aabb(&aabb), None);
	assert_eq!(infinite.get_aabb(&aabb), Some(aabb));
}

#[test]
fn test_get_regions() {
	let params = NoiseParams::new(200, 300);
	let nodes = [
		Node::new(100, 150, Some(50)),
		Node::new(0, 0, Some(50)),
		Node::new(200, 300, Some(50)),
		Node::new(210, 310, Some(50)),
		Node::new(300, 400, Some(50)),
		Node::new(5000, 5000, None),
	];

	let regions = get_regions(params, &nodes);

	assert_eq!(
		regions,
		vec![
			(BoundingBox::new(75, 125, 50, 50), vec![0]),
			(BoundingBox::new(0, 0, 25, 25), vec![1]),
			(BoundingBox::new(175, 275, 25, 25), vec![2]),
			(BoundingBox::new(185, 285, 15, 15), vec![3]),
			(BoundingBox::new(0, 0, 200, 300), vec![5]),
		]
	);
}
