use rand::rngs::ThreadRng;
use serde::Serialize;

use crate::{Orientation, Point2d};

#[derive(Debug, Serialize)]
pub struct Block {
    pub text: String,
    pub top_right: Point2d,
    pub bottom_left: Point2d,
    pub orientation: Orientation,
}

// converts the height of a character to its width
//
// TIMES NEW ROMAN
static W_TO_H_RATIO: f32 = 50_f32 / 83_f32;

impl Block {
    pub(crate) fn new_randomize_orientation(
        text: String,
        area: f32,
        origin: Point2d,
        rng: &mut ThreadRng,
    ) -> Self {
        let orientation = Orientation::at_random(rng);
        Self::new(text, area, origin, orientation)
    }

    pub(crate) fn new(text: String, area: f32, origin: Point2d, orientation: Orientation) -> Self {
        let text_height = Self::h(area, text.len() as f32);
        let text_width = area / text_height;

        let (bottom_left, top_right) = match orientation {
            Orientation::Horizontal => {
                // Horizontal text
                // zero rotation.
                // text pivot points is bottom-left
                let bottom_left = origin.clone();

                let top_right = Point2d {
                    x: origin.x + text_width,
                    y: origin.y - text_height,
                };
                (bottom_left, top_right)
            }
            Orientation::Vertical90 => {
                // Downwards text
                // rotate text 90 degress and text pivot point is
                // the top right corner.
                let bottom_left = Point2d {
                    x: origin.x,
                    y: origin.y + text_width,
                };
                let top_right = Point2d {
                    x: origin.x + text_height,
                    y: origin.y,
                };

                (bottom_left, top_right)
            }
            Orientation::Vertical270 => {
                // Upwards text
                // rotate text 90 degrees clockwise and text pivot point is
                // the bottom right corner.
                let bottom_left = Point2d {
                    x: origin.x - text_height,
                    y: origin.y,
                };
                let top_right = Point2d {
                    x: origin.x,
                    y: origin.y - text_width,
                };

                (bottom_left, top_right)
            }
        };

        Block {
            text,
            bottom_left,
            top_right,
            orientation,
        }
    }

    // Compute the height/font-size given area and the number of characters.
    fn h(area: f32, n_chars: f32) -> f32 {
        let h2 = area / (W_TO_H_RATIO * n_chars);
        h2.sqrt()
    }

    fn is_inside(&self, point: &Point2d) -> bool {
        self.bottom_left.x < point.x
            && self.top_right.x > point.x
            && self.bottom_left.y > point.y
            && self.top_right.y < point.y
    }

    // Test the candidate block against the object under test.
    // Look North, East, South, West for intersection.
    pub(crate) fn is_overlapping(&self, b: &Self) -> bool {
        // if B is always above self.
        if self.top_right.y > b.bottom_left.y {
            return false;
        }

        // if B is always right of self.
        if self.top_right.x < b.bottom_left.x {
            return false;
        }

        // if B is always below self.
        if self.bottom_left.y < b.top_right.y {
            return false;
        }

        // If b is always left of self.
        if self.bottom_left.x > b.top_right.x {
            return false;
        }

        true
    }
}

// let a = vec![
//   (String::from("TAXI"), 24. * 24. * 6.),
//   // All A's have the same area per char.
//   (String::from("A"), 1. * 24. * 24. * 5.),
//   (String::from("AA"), 2. * 24. * 24. * 5.),
//   (String::from("AAA"), 3. * 24. * 24. * 5.),
//   (String::from("AAAAAAAAAA"), 10. * 24. * 24. * 5.)
// ];

#[cfg(test)]
mod test {
    use crate::{Orientation, Point2d};

    use super::Block;

    #[test]
    fn inside() {
        static VALUES: [(Point2d, bool); 5] = [
            // Center
            (
                Point2d {
                    x: 150_f32,
                    y: 150_f32,
                },
                true,
            ),
            // North
            (
                Point2d {
                    x: 150_f32,
                    y: 50_f32,
                },
                false,
            ),
            // East
            (
                Point2d {
                    x: 250_f32,
                    y: 150_f32,
                },
                false,
            ),
            // South
            (
                Point2d {
                    x: 150_f32,
                    y: 350_f32,
                },
                false,
            ),
            // West
            (
                Point2d {
                    x: 50_f32,
                    y: 150_f32,
                },
                false,
            ),
        ];

        let block: Block = Block {
            text: String::from('M'),
            top_right: Point2d {
                x: 200_f32,
                y: 100_f32,
            },
            bottom_left: Point2d {
                x: 100_f32,
                y: 200_f32,
            },
            orientation: Orientation::Horizontal,
        };

        for (p, expected) in &VALUES {
            println!("go");
            assert_eq!(block.is_inside(p), *expected)
        }
    }
}
