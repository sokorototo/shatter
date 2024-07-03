use minifb::{Key, Window, WindowOptions};
use shatter::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
	let mut buffer = vec![0xFFF0F0F0; WIDTH * HEIGHT];

	let mut options = WindowOptions::default();
	options.resize = false;
	options.topmost = true;

	let mut window = Window::new("Worley Noise Demo", WIDTH, HEIGHT, options).unwrap();
	window.set_target_fps(30);

	// Get arena
	let base = BoundingBox::new(0, 0, WIDTH as _, HEIGHT as _);
	let mut nodes = [
		Node::new(0, 0, Some((550, 210))),
		Node::new(75, 150, Some((25, 30))),
		Node::square(400, 400, Some(100)),
		Node::new(410, 350, Some((100, 70))),
		Node::square(150, 150, Some(300)),
		Node::square(350, 400, Some(75)),
		Node::new(450, 200, Some((100, 75))),
		Node::new(150, 250, Some((60, 250))),
		Node::new(400, 300, Some((100, 250))),
	];

	// sorting the nodes in descending order of their influence, massive performance boost
	nodes.sort_by(|a, b| (b.half_extents.as_ref().map(|(x, y)| x * y).unwrap_or(0)).cmp(&a.half_extents.as_ref().map(|(x, y)| x * y).unwrap_or(0)));

	// Get regions
	let mut partitions = get_regions(&base, &nodes);

	// only draw regions with a certain amount of influence
	let mut rendered_regions: usize = nodes.len();

	// Update window
	while window.is_open() && !window.is_key_down(Key::Escape) {
		// controls
		if window.is_key_released(Key::Right) {
			rendered_regions = rendered_regions.saturating_add(1).min(nodes.len());
			partitions = get_regions(&base, &nodes[..rendered_regions]);
		}

		if window.is_key_released(Key::Left) {
			rendered_regions = rendered_regions.saturating_sub(1);
			partitions = get_regions(&base, &nodes[..rendered_regions]);
		}

		// draw
		fill(&nodes, &partitions, &mut buffer);

		// set window buffer
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}

fn fill(nodes: &[Node], partitions: &[(BoundingBox, RcVec<usize>)], buffer: &mut [u32]) {
	// Clear screen to black
	buffer.fill(0);

	for (region, influence) in partitions {
		let influence = influence.as_slice();

		for y in region.top..region.bottom {
			for x in region.left..region.right {
				// get closest node
				let node = if influence.len() == 1 {
					nodes.get(influence[0]).unwrap()
				} else if let Some(node) = {
					let affected = nodes.iter().enumerate().filter(|(idx, _)| influence.contains(idx)).map(|(_, n)| n);
					affected.max_by(|n1, n2| ((n1.x - x).pow(2) + (n1.y - y).pow(2)).cmp(&((n2.x - x).pow(2) + (n2.y - y).pow(2))))
				} {
					node
				} else {
					continue;
				};

				// calculate distance
				let square = (node.x - x).pow(2) + (node.y - y).pow(2);
				let distance = (square as f32).sqrt() as u32;

				buffer[y as usize * WIDTH + x as usize] = distance;
			}
		}
	}
}
