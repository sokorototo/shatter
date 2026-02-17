use std::{time::Instant, vec::Vec};

use shatter::*;
use simplerand::rand_range;

const WIDTH: isize = 800;
const HEIGHT: isize = 800;

const X: isize = -400;
const Y: isize = -400;

const COUNT: u32 = 30;
const ITERATIONS: u32 = 100000;

fn main() {
	// set seed for consistency between runs
	simplerand::set_seed::<u8>(902375098175089174589u128);

	// define region
	let arena = BoundingBox::new(X, Y, WIDTH, HEIGHT);
	let nodes = (0..COUNT)
		.map(|_| {
			let x = rand_range(X, X + WIDTH);
			let y = rand_range(Y, Y + HEIGHT);

			let width = rand_range(50, 200);
			let height = rand_range(50, 200);

			Node::new(x, y, Some((width, height)))
		})
		.collect::<Vec<_>>();

	// benchmark
	let mut regions = Vec::with_capacity(ITERATIONS as usize);
	let then = Instant::now();
	regions.extend((0..ITERATIONS).map(|_| get_regions(&arena, &nodes)));

	let elapsed = then.elapsed();
	println!("Total = {:?}, get_regions took: {:?}", elapsed, elapsed / ITERATIONS);
}
