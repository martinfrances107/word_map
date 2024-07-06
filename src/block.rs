use core::fmt::{self, Display};

use crate::{Orientation, Point2d};

#[derive(Debug)]
pub(crate) struct Block<'a> {
    pub(crate) area: f32,
    // pub(crate) width: f32,
    // pub(crate) height: f32,
    pub(crate) text: &'a str,
    pub(crate) top_right: Point2d,
    pub(crate) bottom_left: Point2d,
    pub(crate) orientation: Orientation,
}

impl<'a> Block<'a> {
    fn is_inside(&self, point: &Point2d) -> bool {
        // println!(
        //     "x test {} {} - px {} {}",self.bottom_left.x, self.top_right.x , point.x,
        //     self.bottom_left.x < point.x && self.top_right.x > point.x
        // );
        // println!(
        //     "y test {}",
        //     self.bottom_left.y > point.y && self.top_right.y < point.y
        // );
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
impl<'a> Display for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.top_right.x - self.bottom_left.x;
        let height = self.bottom_left.y - self.top_right.y;

        let rect_x = self.bottom_left.x;
        let rect_y = self.bottom_left.y - height;

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
            rect_x, rect_y, width, height
        )?;
        writeln!(
            f,
            "<text x=\"{}\" y=\"{}\" >{}</text>",
            self.bottom_left.x, self.bottom_left.y, self.text
        )
    }
}

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

        static BLOCK: Block<'static> = Block {
            area: 100_f32 * 100_f32,
            // width: 100_f32,
            // height: 100_f32,
            text: &"M",
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
