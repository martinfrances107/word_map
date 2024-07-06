use rand::{rngs::ThreadRng, Rng};

use crate::{block::Block, Orientation, Point2d};

pub(crate) struct Grid<'a> {
    rng: ThreadRng,
    width: f32,
    height: f32,
    // top_right: Point2d,
    // bottom_left: Point2d,
    pub(crate) blocks: Vec<Block<'a>>,
}

impl<'a> Grid<'a> {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self {
            rng: rand::thread_rng(),
            // bottom_left: Point2d { x: 0_f32, y: 0_f32 },
            // top_right: Point2d {
            //     x: width,
            //     y: height,
            // },
            blocks: vec![],
            width,
            height,
        }
    }
}
impl<'a> Grid<'a> {
    fn is_inside(&self, p: &Point2d) -> bool {
        p.x > 0_f32 && p.x < self.width && p.y > 0_f32 && p.y < self.height
    }

    fn point_at_random(&mut self) -> Point2d {
        let x = self.rng.gen_range(0_f32..self.width);
        let y = self.rng.gen_range(0_f32..self.height);
        Point2d { x, y }
    }

    // Generate candidate blocks and fit them into the GRID.
    //
    // WARNING:
    // O(n^2) operation
    pub(crate) fn place_block(&mut self, text: &'a str, area: f32) -> bool {
        // Give a block 3x100 attempts to get placed.
        // 2 orientations
        for _ in 0..100 {
            let origin = self.point_at_random();
            let mut candidate = Block::new(text, area, origin, Orientation::Horizontal);
            // Block must be inside the grid/canvas.
            if self.is_inside(&candidate.bottom_left) || self.is_inside(&candidate.top_right) {
                if !self.is_overlapping(&candidate) {
                    self.blocks.push(candidate);
                    return true;
                } else {
                    candidate.orientation = Orientation::Vertical;
                    if !self.is_overlapping(&candidate) {
                        self.blocks.push(candidate);
                        return true;
                    }
                }
            }
        }
        false
    }

    // Loop over
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
