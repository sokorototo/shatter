## shatter: A dumb 2D space allocation algorithm

Given a 2D space defined by an Axis Aligned Bounding Box (AABB) and an 0-indexed array of `&[Nodes]`, a node being a 2D point anywhere and an optional area of influence, `shatter` returns a list on non-intersecting Axis Aligned Bounding Boxes with each AABB containing a list of indexes to `Nodes` that contest influence within the AABB. Find below a demonstration image:

![Space Division Demo](<demos/demo.avif>)

Above, a 600x600 space (The Window's draw area) is divided among several AABBs. Each AABB contains a list of indexes the the `Nodes` list initial input.

### Uses

I don't know, I wasted one weekend thinking this shit out. I wrote it as an optimization to Worley Noise generation on the CPU. A percentage of the remaining space is unallocated and effectively work that's discarded by the eventual pixel filling algorithm. For the allocated AABBs, it a comparison for the closes pixel in a very small array (with 30 nodes the max AABB index size is about 7 and most AABBs are in the 2-4 range). That aside, knock yourself out finding uses for this shit 😆

### TODO
 - Make `BoundingBox` struct generic over a numerical type, or just use `f32` instead of `isize`