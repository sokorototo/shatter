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
	let mut window = Window::new("Worley Noise Demo", WIDTH, HEIGHT, options).unwrap();
	window.set_target_fps(30);

	// define arena
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
}
