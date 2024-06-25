use std::{hint::black_box, time::Instant, vec::Vec};

use shatter::*;
use simplerand::rand_range;

const WIDTH: isize = 800;
const HEIGHT: isize = 800;

const X: isize = -400;
const Y: isize = -400;

const COUNT: u32 = 50;
const ITERATIONS: u32 = 10000;

fn main() {
	// define region
	let arena = BoundingBox::new(X, Y, WIDTH, HEIGHT);
	let base = BoundingBox::new(0, 0, WIDTH as _, HEIGHT as _);
	let mut nodes = (0..COUNT)
		.map(|_| {
			let x = rand_range(X as isize, X as isize + WIDTH as isize);
			let y = rand_range(Y as isize, Y as isize + HEIGHT as isize);

			let width = rand_range(50, 200);
			let height = rand_range(50, 200);

			Node::new(x, y, Some((width, height)))
		})
		.collect::<Vec<_>>();


	// benchmark
	let then = Instant::now();
	for _ in 0..ITERATIONS {
		let regions = get_regions(&arena, &nodes);
		let _ = black_box(regions);
	}
	println!("get_regions took: {:?}", then.elapsed() / ITERATIONS);
}
