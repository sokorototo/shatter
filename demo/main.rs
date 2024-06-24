use std::hash::{DefaultHasher, Hash, Hasher};

use minifb::{Key, Window, WindowOptions};
use shatter::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const TEXT_PADDING: usize = 3;

fn main() {
	let mut buffer = vec![0xFFF0F0F0; WIDTH * HEIGHT];

	let mut options = WindowOptions::default();
	options.resize = false;
	options.topmost = true;

	// Limit to max ~60 fps update rate
	let mut window = Window::new("ESC to exit, Up/Down incr or decr influence, Left/Right adds/removes Nodes", WIDTH, HEIGHT, options).unwrap();
	window.set_target_fps(30);

	let font = minifb_fonts::font6x8::new_renderer(WIDTH, HEIGHT, 0xFF000000);

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
	let mut regions = get_regions(&base, &nodes);
	let mut max_influence = regions.iter().map(|(_, i)| i.len()).max().unwrap_or(0);

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
			shown_influence = shown_influence.saturating_add(1).min(max_influence);
		}

		if window.is_key_released(Key::Down) {
			shown_influence = shown_influence.saturating_sub(1);
		}

		if window.is_key_released(Key::Right) {
			rendered_regions = rendered_regions.saturating_add(1).min(nodes.len());
			regions = get_regions(&base, &nodes[..rendered_regions]);
			max_influence = regions.iter().map(|(_, i)| i.len()).max().unwrap_or(0);
		}

		if window.is_key_released(Key::Left) {
			rendered_regions = rendered_regions.saturating_sub(1);
			regions = get_regions(&base, &nodes[..rendered_regions]);
			max_influence = regions.iter().map(|(_, i)| i.len()).max().unwrap_or(0);
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

			// draw influence
			let mut hasher = DefaultHasher::new();
			influence.hash(&mut hasher);
			let hash = hasher.finish() as u32;

			for y in region.top..region.bottom {
				let y = y as usize;
				let slice = buffer.get_mut((y * WIDTH + offset)..(y * WIDTH + offset + width)).unwrap();
				slice.fill(hash);
			}

			// draw text
			let text = format!("{:?}", influence);
			font.draw_text(&mut buffer, region.left as usize + TEXT_PADDING, region.top as usize + TEXT_PADDING, &text);

			// draw border
			let top_line = buffer
				.get_mut((region.top as usize * WIDTH + offset)..(region.top as usize * WIDTH + offset + width))
				.unwrap();
			top_line.fill(0xFF000000);

			let bottom_line = buffer
				.get_mut((region.bottom as usize * WIDTH + offset)..(region.bottom as usize * WIDTH + offset + width))
				.unwrap();
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
