#![windows_subsystem = "windows"]

use std::{
	hash::{DefaultHasher, Hash, Hasher},
	mem,
};

use minifb::{Key, Window, WindowOptions};
use shatter::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const TEXT_PADDING: usize = 3;

fn generate_nodes<const N: usize>() -> [Node; N] {
	let mut nodes: [Node; N] = unsafe { mem::MaybeUninit::uninit().assume_init() };

	for node in &mut nodes {
		*node = match simplerand::rand::<u8>() % 2 == 0 {
			// generate a square node
			true => {
				let half_extent = Some(simplerand::rand_range::<isize>(25, 125));

				let x = simplerand::rand_range(5, (WIDTH - 5) as _);
				let y = simplerand::rand_range(5, (HEIGHT - 5) as _);

				Node::square(x, y, half_extent)
			}
			// generate a rectangular node
			false => {
				let half_extents = {
					let hx = simplerand::rand_range::<isize>(25, 125);
					let hy = simplerand::rand_range::<isize>(25, 125);

					Some((hx, hy))
				};

				let x = simplerand::rand_range(5, (WIDTH - 5) as _);
				let y = simplerand::rand_range(5, (HEIGHT - 5) as _);

				Node::new(x, y, half_extents)
			}
		};
	}

	nodes
}

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
	let mut nodes = generate_nodes::<20>();

	// sort to reduce fragmentation
	nodes.sort_by(|a, b| (b.half_extents.as_ref().map(|(x, y)| x * y).unwrap_or(0)).cmp(&a.half_extents.as_ref().map(|(x, y)| x * y).unwrap_or(0)));

	// Get regions
	let mut regions = get_regions(&base, &nodes);
	let mut max_influence = regions.iter().map(|(_, i)| i.len()).max().unwrap_or(0);

	// only draw regions with a certain amount of influence
	let mut rendered_regions: usize = nodes.len() / 2;
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
		}

		if window.is_key_released(Key::R) {
			nodes = generate_nodes();
			regions = get_regions(&base, &nodes[..rendered_regions]);
			max_influence = regions.iter().map(|(_, i)| i.len()).max().unwrap_or(0);
			shown_influence = shown_influence.min(max_influence);
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
				let slice = buffer.get_mut((y * WIDTH + offset)..(y * WIDTH + (offset + width).min(WIDTH))).unwrap();
				slice.fill(hash);
			}

			// draw text
			let text = format!("{:?}", influence);
			font.draw_text(&mut buffer, region.left as usize + TEXT_PADDING, region.top as usize + TEXT_PADDING, &text);

			// draw borders
			if let Some(t) = buffer.get_mut((region.top as usize * WIDTH + offset)..(region.top as usize * WIDTH + offset + width)) {
				t.fill(0xFF000000)
			};

			if let Some(b) = buffer.get_mut((region.bottom as usize * WIDTH + offset)..(region.bottom as usize * WIDTH + offset + width)) {
				b.fill(0xFF000000)
			};

			for y in region.top..region.bottom {
				buffer.get_mut(y as usize * WIDTH + region.left as usize).map(|b| *b = 0xFF000000);
				buffer.get_mut(y as usize * WIDTH + region.right as usize).map(|b| *b = 0xFF000000);
			}
		}

		// set window buffer
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}
