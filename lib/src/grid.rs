use rand::{rngs::ThreadRng, Rng};

use crate::{block::Block, Orientation, Point2d};

pub struct Grid {
    rng: ThreadRng,
    width: f32,
    height: f32,
    pub blocks: Vec<Block>,
    // Bounding rectangle
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
}

impl Grid {
    /// Returns a grid object given the dimension of the canvas/svg
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rng: rand::thread_rng(),
            blocks: vec![],
            width,
            height,
            xmin: 0_f32,
            xmax: width,
            ymin: 0_f32,
            ymax: height,
        }
    }

    /// Returns the sub rectangle.
    pub fn bounding_rectangle(&mut self) -> (f32, f32, f32, f32) {
        (self.xmin, self.xmax, self.ymin, self.ymax)
    }

    /// Constrain the sub rectangle.
    ///
    /// no addition blocks with be placed outside the rectangle.
    pub fn bounding_rectangle_set(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32) {
        debug_assert!(xmin > 0_f32);
        debug_assert!(ymin > 0_f32);
        debug_assert!(xmax > xmin);
        debug_assert!(ymax > ymin);
        debug_assert!(self.width > xmax);
        debug_assert!(self.height > ymax);
        self.xmin = xmin;
        self.xmax = xmax;
        self.ymin = ymin;
        self.ymax = ymax;
    }

    // Is a point inside the bounding rectangle.
    fn is_inside(&self, p: &Point2d) -> bool {
        p.x > self.xmin && p.x < self.xmax && p.y > self.ymin && p.y < self.ymax
    }

    // Point is limited to the bounding rectangle.
    fn point_at_random(&mut self) -> Point2d {
        let x = self.rng.gen_range(self.xmin..self.xmax);
        let y = self.rng.gen_range(self.ymin..self.ymax);
        Point2d { x, y }
    }

    /// Generate candidate blocks and fit them into the bounding rectangle.
    ///
    /// WARNING:
    /// O(n^2) operation
    pub fn place_block(&mut self, text: String, area: f32) -> bool {
        // Give a block 3x100 attempts to get placed.
        // 2 orientations
        for _ in 0..100 {
            let origin = self.point_at_random();
            let mut candidate =
                Block::new_randomize_orientation(text.clone(), area, origin, &mut self.rng);
            // Block must be inside the grid/canvas.
            if self.is_inside(&candidate.bottom_left) && self.is_inside(&candidate.top_right) {
                if !self.is_overlapping(&candidate) {
                    self.blocks.push(candidate);
                    return true;
                } else {
                    candidate.orientation = Orientation::Vertical90;
                    if !self.is_overlapping(&candidate) {
                        // self.blocks.push(candidate);
                        // return true;
                    } else {
                        // Other Vertical Oritienttion downwards/upwards
                    }
                }
            }
        }
        false
    }

    // Check candidate block over all existing blocks.
    fn is_overlapping(&self, test_block: &Block) -> bool {
        for block in &self.blocks {
            // is candidate bottom-left inside block.
            if block.is_overlapping(test_block) {
                return true;
            }
        }
        false
    }
}
