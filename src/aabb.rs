#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundingBox {
	pub left: usize,
	pub right: usize,
	pub top: usize,
	pub bottom: usize,
}

impl BoundingBox {
	pub fn new(x: usize, y: usize, width: usize, height: usize) -> BoundingBox {
		BoundingBox {
			left: x,
			right: x + width,
			top: y,
			bottom: y + height,
		}
	}

	pub fn contains(&self, other: &BoundingBox) -> bool {
		self.left <= other.left && self.top <= other.top && self.right >= other.right && self.bottom >= other.bottom
	}

	pub fn contains_point(&self, x: usize, y: usize) -> bool {
		x >= self.left && x < self.right && y >= self.top && y < self.bottom
	}

	pub fn intersects(&self, other: &BoundingBox) -> bool {
		let x_intersect = self.right > other.left && self.left < other.right;
		let y_intersect = self.bottom > other.top && self.top < other.bottom;

		x_intersect && y_intersect
	}

	pub fn intersection(&self, other: &BoundingBox) -> Option<BoundingBox> {
		self.intersects(other).then(|| {
			let x = self.left.max(other.left);
			let y = self.top.max(other.top);

			let x2 = self.right.min(other.right);
			let y2 = self.bottom.min(other.bottom);

			BoundingBox {
				left: x,
				top: y,
				right: x2,
				bottom: y2,
			}
		})
	}

	// subtracts the other bounding box from this bounding box
	pub fn difference<F: FnOnce(&[BoundingBox])>(&self, rhs: &BoundingBox, callback: F) {
		// if the 2 boxes don't intersect then there is no subtraction
		if !self.intersects(rhs) {
			return callback(&[]);
		}

		let mut regions: [BoundingBox; 4] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
		let mut idx = 0;

		// chopping based algorithm
		let mut base = self.clone();

		// top
		if base.top < rhs.top {
			regions[idx] = BoundingBox { bottom: rhs.top, ..base };
			base.top = rhs.top;

			idx += 1;
		}

		// bottom
		if base.bottom > rhs.bottom {
			regions[idx] = BoundingBox { top: rhs.bottom, ..base };
			base.bottom = rhs.bottom;

			idx += 1;
		}

		// middle-left
		if base.left < rhs.left {
			regions[idx] = BoundingBox { right: rhs.left, ..base };
			base.left = rhs.left;

			idx += 1;
		}

		// middle-right
		if base.right > rhs.right {
			regions[idx] = BoundingBox { left: rhs.right, ..base };
			base.right = rhs.right;

			idx += 1;
		}

		callback(&regions[..idx]);
	}
}
