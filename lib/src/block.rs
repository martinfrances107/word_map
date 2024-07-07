use core::fmt::{self, Display};

use rand::rngs::ThreadRng;

use crate::{Orientation, Point2d};

#[derive(Debug)]
pub struct Block {
    pub(crate) area: f32,
    pub(crate) text: String,
    pub(crate) top_right: Point2d,
    pub(crate) bottom_left: Point2d,
    pub(crate) orientation: Orientation,
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
            area,
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

// Outputs a String containing SVG elements.
//
// Coord system translation.
// for performance bottom-left is used for the the text x,y without further computation.
// rectangle and circle points further need computation.
impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // rec width is not text width.
        let rec_width = self.top_right.x - self.bottom_left.x;
        // rec_height is not text height.
        let rec_height = self.bottom_left.y - self.top_right.y;

        // top left
        let rect_x = self.bottom_left.x;
        let rect_y = self.bottom_left.y - rec_height;

        writeln!(
            f,
            "<circle class=\"bl\" cx=\"{}\" cy=\"{}\" stroke=\"blue\" r=\"2\" />",
            self.bottom_left.x, self.bottom_left.y
        )?;
        writeln!(
            f,
            "<circle clas=\"tr\" cx=\"{}\" cy=\"{}\" stroke=\"blue\" r=\"2\" />",
            self.top_right.x, self.top_right.y
        )?;
        writeln!(
            f,
            "<rect x=\"{}\" y=\"{}\" fill=\"none\" stroke=\"red\" width=\"{}\" height=\"{}\"/>",
            rect_x, rect_y, rec_width, rec_height
        )?;

        match self.orientation {
            Orientation::Horizontal => {
                writeln!(
                    f,
                    "<text transform=\"translate({}, {}) rotate(0)\" font-size=\"{}\" >{}</text>",
                    self.bottom_left.x, self.bottom_left.y, rec_height, self.text
                )
            }
            Orientation::Vertical90 => {
                // origin is top left
                let top_left = Point2d {
                    x: self.top_right.x - rec_width,
                    y: self.top_right.y,
                };
                writeln!(
                    f,
                    "<text transform=\"translate({}, {}) rotate(90)\" font-size=\"{}\" >{}</text>",
                    top_left.x, top_left.y, rec_width, self.text
                )
            }
            Orientation::Vertical270 => {
                // origin is bottom right
                let bottom_right = Point2d {
                    x: self.bottom_left.x + rec_width,
                    y: self.bottom_left.y,
                };
                writeln!(
                    f,
                    "<text transform=\"translate({}, {}) rotate(270)\" font-size=\"{}\" >{}</text>",
                    bottom_right.x, bottom_right.y, rec_width, self.text
                )
            }
        }
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

        static BLOCK: Block = Block {
            area: 100_f32 * 100_f32,
            // width: 100_f32,
            // height: 100_f32,
            text: String("M"),
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
            assert_eq!(BLOCK.is_inside(p), *expected)
        }
    }
}
