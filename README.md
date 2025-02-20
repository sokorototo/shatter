<h4 align=center>🥃 shatter: A 2D space allocation algorithm allowing intersecting pages</h4>
<p align=center>
  <img alt="GitHub License" src="https://img.shields.io/github/license/sokorototo/shatter?style=flat-square">
  <a href="https://github.com/sokorototo/shatter/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues-raw/sokorototo/shatter?style=flat-square"></a>
</p>

Given a 2D space defined by an Axis Aligned Bounding Box (AABB) and an 0-indexed array of `&[Nodes]`, a node being a 2D point anywhere and an optional area of influence, `shatter` returns a list on non-intersecting Axis Aligned Bounding Boxes with each AABB containing a list of indexes to `Nodes` that contest influence within the AABB. Find below a demonstration image:

 - ![Space Division Demo](<demos/demo.avif>)
 - ###### Regions with the same influence list have the same colour.

Above, a 600x600 space (The Window's draw area) is divided among several AABBs. Each AABB contains a list of indices to the `Nodes` list initial input.

### 🧪 Sample Usage

The example below basically describes the setup for the above image, minus rendering.

```rust
use shatter::*;

// define 600x600 space
const WIDTH: isize = 600;
const HEIGHT: isize = 600;

// space goes into the negatives
const X: isize = -200;
const Y: isize = -200;

fn main() {
   // define region
   let arena = BoundingBox::new(X, Y, WIDTH, HEIGHT);

   // define Nodes
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

   // generate regions
   let regions = get_regions(&arena, &nodes);
   for (region, influence) in regions {
      // `influence` is a custom type, use `as_ref` to get a slice into the data
      let influence = influence.as_ref();
      println!("Region: {:?}, Influenced Nodes: {:?}", &region, influence);
   }
}
```

### 💭 Uses

I don't know, I wasted one weekend thinking this shit out. I wrote it as an optimization to Worley Noise generation on the CPU. A percentage of the remaining space is unallocated and effectively work that's discarded by the eventual pixel filling algorithm. For the allocated AABBs, it a comparison for the closes pixel in a very small array (with 30 nodes the max AABB index size is about 7 and most AABBs are in the 2-4 range). That aside, knock yourself out finding uses for this shit 😆

## ⚙️ How it works

#### \> WIP 🚧 <

### 📃 TODO
 - Make `BoundingBox` struct generic over a numerical type, or just use `f32` instead of `isize`
 - Fix `noise` example