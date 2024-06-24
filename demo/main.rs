use std::hash::{DefaultHasher, Hash, Hasher};

use minifb::{Key, Window, WindowOptions};
use shatter::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
	let mut buffer = vec![0xFFF0F0F0; WIDTH * HEIGHT];

	let mut options = WindowOptions::default();
	options.resize = false;
	options.topmost = true;

	// Limit to max ~60 fps update rate
	let mut window = Window::new("ESC to exit, Up/Down incr or decr influence, Left/Right adds/removes Nodes", WIDTH, HEIGHT, options).unwrap();
	window.set_target_fps(30);

	// Get arena
	let base = BoundingBox::new(0, 0, WIDTH as _, HEIGHT as _);
	let mut nodes = [
		Node::square(200, 200, Some(100)),
		Node::square(0, 0, Some(200)),
		Node::square(75, 150, Some(25)),
		Node::square(400, 400, Some(100)),
		Node::square(410, 350, Some(100)),
		Node::square(150, 150, Some(300)),
		Node::square(350, 400, Some(75)),
		Node::square(450, 200, Some(100)),
		Node::square(150, 250, Some(60)),
	];

	// sorting the nodes in descending order of their influence, massive performance boost
	nodes.sort_by(|a, b| b.half_extents.cmp(&a.half_extents));

	// Get regions
	let mut regions = get_regions(&base, &nodes);

	let then = std::time::Instant::now();
	for _ in 0..1000 {
		let _regions = get_regions(&base, &nodes);
		std::mem::forget(_regions);
	}
	println!("get_regions took: {:?}, len: {}", then.elapsed() / 1000, regions.len());

	// only draw regions with a certain amount of influence
	let mut rendered_regions: usize = nodes.len();
	let mut shown_influence: usize = 0;

	// Update window
	while window.is_open() && !window.is_key_down(Key::Escape) {
		// controls
		if window.is_key_released(Key::Up) {
			shown_influence = shown_influence.saturating_add(1);
		}

		if window.is_key_released(Key::Down) {
			shown_influence = shown_influence.saturating_sub(1);
		}

		if window.is_key_released(Key::Right) {
			rendered_regions = rendered_regions.saturating_add(1).min(nodes.len());
			regions = get_regions(&base, &nodes[..rendered_regions]);
		}

		if window.is_key_released(Key::Left) {
			rendered_regions = rendered_regions.saturating_sub(1);
			regions = get_regions(&base, &nodes[..rendered_regions]);
		}

		// clear screen
		buffer.fill(0xFFF0F0F0);

		// draw illustration
		for (region, influence) in &regions {
			if influence.len() != shown_influence && shown_influence != 0 {
				continue;
			}

			let width = (region.right - region.left) as usize;
			let offset = region.left as usize;

			let mut hasher = DefaultHasher::new();
			influence.hash(&mut hasher);
			let hash = hasher.finish() as u32;

			for y in region.top..region.bottom {
				let y = y as usize;
				let slice = &mut buffer[(y * WIDTH + offset)..(y * WIDTH + offset + width)];
				slice.fill(hash);
			}

			let top_line = &mut buffer[(region.top as usize * WIDTH + offset)..(region.top as usize * WIDTH + offset + width)];
			top_line.fill(0xFF000000);

			let bottom_line = &mut buffer[(region.bottom as usize * WIDTH + offset)..(region.bottom as usize * WIDTH + offset + width)];
			bottom_line.fill(0xFF000000);

			for y in region.top..region.bottom {
				let y = y as usize;
				buffer[y * WIDTH + region.left as usize] = 0xFF000000;
				buffer[y * WIDTH + region.right as usize] = 0xFF000000;
			}
		}

		// set window buffer
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}
