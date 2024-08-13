use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use rand::{rngs::ThreadRng, Rng};

use crate::{block::Block, Point2d};

/// Parser only structure.
#[derive(Debug, Eq, PartialEq)]
pub struct TextWeight<'a>(pub &'a str, pub u32);

/// A collection of blocks.
#[derive(Debug)]
pub struct Grid {
    rng: ThreadRng,
    width: f32,
    height: f32,
    /// A collection of blocks.
    pub blocks: Vec<Block>,
    // Bounding rectangle
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
}

impl Grid {
    /// Returns a grid object given the dimension of the canvas/svg
    #[must_use]
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
    #[must_use]
    pub const fn bounding_rectangle(&self) -> (f32, f32, f32, f32) {
        (self.xmin, self.xmax, self.ymin, self.ymax)
    }

    /// Reset placement to the full surface.
    pub fn bounding_rectangle_clear(&mut self) {
        self.xmin = 0_f32;
        self.xmax = self.width;
        self.ymin = 0_f32;
        self.ymax = self.height;
    }

    /// Constrain the sub rectangle.
    ///
    /// All blocks with be placed inside the rectangle.
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

    /// Generate candidate blocks and fit them into the bounding rectangle.
    ///
    /// WARNING:
    /// O(n^2) operation
    pub fn place_block(&mut self, text: &str, area: f32) -> bool {
        // Give a block 2000 attempts to get placed.
        // 2 orientations
        for _ in 0..2000 {
            let origin = self.point_at_random();
            let block =
                Block::new_randomize_orientation(text.to_string(), area, &origin, &mut self.rng);
            // Block must be inside the bounding rectangle.
            if self.is_inside(&block.bottom_left)
                && self.is_inside(&block.top_right)
                && !self.is_any_block_overlapping(&block)
            {
                self.blocks.push(block);
                return true;
            }
        }
        false
    }

    /// Converts a string into list of (text,weight) pairs.
    ///
    /// For example "apple,2 bubble,10"
    ///
    /// This parse list can then added to the grid using `place_block`.
    pub fn parse_pairs(input: &str) -> IResult<&str, Vec<TextWeight>> {
        separated_list1(char(' '), Self::parse_text_weight)(input)
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

    // Check candidate block over all existing blocks.
    fn is_any_block_overlapping(&self, test_block: &Block) -> bool {
        for block in &self.blocks {
            // is candidate bottom-left inside block.
            if block.is_overlapping(test_block) {
                return true;
            }
        }
        false
    }

    fn parse_text_weight(input: &str) -> IResult<&str, TextWeight> {
        let parse_pair = separated_pair(alpha1::<&str, _>, char(','), digit1);

        map(parse_pair, |(text, weight_str)| {
            let weight = weight_str.parse::<_>().expect("must see valid u32");
            TextWeight(text, weight)
        })(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_text_weight() {
        assert_eq!(
            Grid::parse_text_weight("apple,2"),
            Ok(("", TextWeight("apple", 2)))
        );
    }

    #[test]
    fn parse_list() {
        let expected = vec![TextWeight("apple", 2), TextWeight("bubble", 10)];
        assert_eq!(Grid::parse_pairs("apple,2 bubble,10"), Ok(("", expected)));
    }
}
