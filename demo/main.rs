use minifb::{Key, Window, WindowOptions};
use shatter::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
	let mut buffer = vec![0; WIDTH * HEIGHT];

	let mut options = WindowOptions::default();
	options.resize = false;
	options.topmost = true;

	// Limit to max ~60 fps update rate
	let mut window = Window::new("Draw Cutting Algorithm - ESC to exit", WIDTH, HEIGHT, options).unwrap();
	window.set_target_fps(30);

	// Get arena
	let base = BoundingBox::new(0, 0, WIDTH as _, HEIGHT as _);
	let nodes = [Node::new(75, 150, Some(50)), Node::new(75, 150, Some(25))];

	while window.is_open() && !window.is_key_down(Key::Escape) {
		for i in buffer.iter_mut() {
			*i = 0; // write something more funny here!
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}
