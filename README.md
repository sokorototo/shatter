<h4 align=center>🥃 shatter: A dumb 2D space allocation algorithm</h4>
<p align=center>
  <img alt="GitHub" src="https://img.shields.io/github/license/sokorototo/shatter?style=flat-square">
  <a href="https://github.com/sokorototo/shatter/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues-raw/sokorototo/shatter?style=flat-square"></a>
</p>

Given a 2D space defined by an Axis Aligned Bounding Box (AABB) and an 0-indexed array of `&[Nodes]`, a node being a 2D point anywhere and an optional area of influence, `shatter` returns a list on non-intersecting Axis Aligned Bounding Boxes with each AABB containing a list of indexes to `Nodes` that contest influence within the AABB. Find below a demonstration image:

![Space Division Demo](<demos/demo.avif>)

Above, a 600x600 space (The Window's draw area) is divided among several AABBs. Each AABB contains a list of indexes the the `Nodes` list initial input.

### 🧪 Sample Usage

```rust
use shatter::*;
use simplerand::rand_range;

// define 800x800 space
const WIDTH: isize = 800;
const HEIGHT: isize = 800;

// space goes into the negatives
const X: isize = -400;
const Y: isize = -400;

// Generate 30 nodes
const COUNT: u32 = 30;

fn main() {
	// define region
	let arena = BoundingBox::new(X, Y, WIDTH, HEIGHT);
	let nodes = (0..COUNT)
		.map(|_| {
			let x = rand_range(X as isize, X as isize + WIDTH as isize);
			let y = rand_range(Y as isize, Y as isize + HEIGHT as isize);

			let width = rand_range(50, 200);
			let height = rand_range(50, 200);

			Node::new(x, y, Some((width, height)))
		})
		.collect::<Vec<_>>();

	let regions = get_regions(&arena, &nodes);
	println!("Generated Regions: {:?}" regions);
}

```

### 💭 Uses

I don't know, I wasted one weekend thinking this shit out. I wrote it as an optimization to Worley Noise generation on the CPU. A percentage of the remaining space is unallocated and effectively work that's discarded by the eventual pixel filling algorithm. For the allocated AABBs, it a comparison for the closes pixel in a very small array (with 30 nodes the max AABB index size is about 7 and most AABBs are in the 2-4 range). That aside, knock yourself out finding uses for this shit 😆

## ⚙️ How it works

#### \> WIP 🚧 <

### 📃 TODO
 - Make `BoundingBox` struct generic over a numerical type, or just use `f32` instead of `isize`