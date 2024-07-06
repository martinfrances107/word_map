mod block;
mod grid;

use crate::block::Block;
use rand::Rng;

#[derive(Clone, Debug)]
struct Point2d {
    x: f32,
    y: f32,
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl<'a> Block<'a> {
    fn new(text: &'a str, area: f32, origin: Point2d, orientation: Orientation) -> Self {
        let width = 24_f32 * text.len() as f32;
        let height = area / width;

        let bottom_left = origin.clone();

        let top_right = Point2d {
            x: origin.x + width,
            y: origin.y - height,
        };

        Block {
            area,
            // width,
            // height,
            text,
            bottom_left,
            top_right,
            orientation,
        }
    }
}

static SCALE: f32 = 1_f32;
static WIDTH: f32 = 800f32;
static HEIGHT: f32 = 600f32;

use crate::grid::Grid;

fn main() {
    use random_word::Lang;
    let mut rng = rand::thread_rng();

    let mut grid = Grid::new(WIDTH, HEIGHT);

    println!("<?xml version=\"1.0\" standalone=\"no\"?><!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg version=\"1.1\"
      width=\"{WIDTH}\"
      height=\"{HEIGHT}\"
      viewBox=\"0 0 1200 518\"
      xmlns=\"http://www.w3.org/2000/svg\"
    >");
    println!("<g>");
    // Assign a random number to a word selected at random
    for b in 0..900 {
        // input range 1..10 ( no zero width )
        //
        // assuming a char width of 24px
        // maps to a screen area based on 24x24 squares
        let area = 24_f32 * 24_f32 * rng.gen_range(1_f32..10_f32);
        let text = random_word::gen(Lang::En);
        grid.place_block(text, area);
    }

    for b in grid.blocks {
        println!("{}", b)
    }
    println!("</g>");
    println!("</svg>");

    // let word = random_word::gen(Lang::En);
}
