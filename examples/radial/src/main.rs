extern crate word_map;

use word_map::{block::Block, Orientation, Point2d};

// use word_map::block::Block;
use rand::distributions::{Distribution, WeightedIndex};
use word_map::grid::Grid;

static WIDTH: f32 = 800f32;
static HEIGHT: f32 = 600f32;

// A hightly skew distriubtion where small areas are
// much more likely that large.
fn render_block(b: &Block) {
    // rec width is not text width.
    let rec_width = b.top_right.x - b.bottom_left.x;
    // rec_height is not text height.
    let rec_height = b.bottom_left.y - b.top_right.y;

    match b.orientation {
        Orientation::Horizontal => {
            println!(
                "<text transform=\"translate({}, {}) rotate(0)\" font-size=\"{}\">{}</text>",
                b.bottom_left.x, b.bottom_left.y, rec_height, b.text
            )
        }
        Orientation::Vertical90 => {
            // origin is top left
            let top_left = Point2d {
                x: b.top_right.x - rec_width,
                y: b.top_right.y,
            };
            println!(
                "<text transform=\"translate({}, {}) rotate(90)\" font-size=\"{}\">{}</text>",
                top_left.x, top_left.y, rec_width, b.text
            )
        }
        Orientation::Vertical270 => {
            // origin is bottom right
            let bottom_right = Point2d {
                x: b.bottom_left.x + rec_width,
                y: b.bottom_left.y,
            };
            println!(
                "<text transform=\"translate({}, {}) rotate(270)\" font-size=\"{}\">{}</text>",
                bottom_right.x, bottom_right.y, rec_width, b.text
            )
        }
    }
}

fn main() {
    use random_word::Lang;
    let mut rng = rand::thread_rng();

    let mut grid = Grid::new(WIDTH, HEIGHT);

    // Parabolic area distribution. 1..81
    let area_values: [f32; 9] = [
        1_f32.powf(2.),
        2_f32.powf(2.),
        3_f32.powf(2.),
        4_f32.powf(2.),
        5_f32.powf(2.),
        6_f32.powf(2.),
        7_f32.powf(2.),
        8_f32.powf(2.),
        9_f32.powf(2.),
    ];
    // Heavily skew towards small areas.
    static AREA_WEIGHTS: [usize; 9] = [100, 50, 1, 1, 1, 1, 1, 1, 1];

    let dist = WeightedIndex::new(AREA_WEIGHTS).unwrap();

    println!("<?xml version=\"1.0\" standalone=\"no\"?><!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg version=\"1.1\"
      width=\"{WIDTH}\"
      height=\"{HEIGHT}\"
      xmlns=\"http://www.w3.org/2000/svg\"
    >");
    println!(
        "<defs>
            <style><![CDATA[
              svg{{
              --white: #f3ffff;
              --red: hsl(0, 100%, 50%);
              }}

              svg{{
                background-color: black;
              }}
              /* bottom left of text block */
              .bl {{
                fill: var(--red);
                stroke: None;
              }}
              #background {{
                fill: url(#RadialBackground)
              }}
              /* top right corner of text block */
              .tr {{
                fill: var(--red);
                stroke: None;

              }}
              rect {{
                stroke: var(--white);
                fill: none
              }}
              text {{
                fill: var(--white);
                font-weight: bold;
              }}
            ]]></style>
            <radialGradient id=\"RadialBackground\" cx=\"50%\" cy=\"50%\" r=\"50%\" fx=\"50%\" fy=\"50%\">
              <stop offset=\"0%\" stop-color=\"#505050\" />
              <stop offset=\"100%\" stop-color=\"black\" />
            </radialGradient>

          </defs>"
    );
    println!("<rect id=\"background\" x=\"0\" y=\"0\" width=\"{WIDTH}\" height=\"{HEIGHT}\" fill=\"url(#RadialBackround)\"/>" );
    println!("<g font-family=\"Courier\">");

    // Assign a random number to a word selected at random
    let mut word_list = (0..200)
        .map(|_| {
            // input range 1..10 ( no zero width )
            // based on a highly skewed algorithm.
            //
            // assuming a char width of 24px
            // maps to a screen area based on 24x24 squares
            let area = 7_f32 * 24_f32 * area_values[dist.sample(&mut rng)];
            let text = random_word::gen(Lang::En).to_uppercase();
            (text, area)
        })
        .collect::<Vec<_>>();
    word_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // bounding rectangle fit the top 5% into 1/9 of the surface area.
    let n_big = 40;

    let sorted_iter = word_list.iter().rev();

    // Limit the grid bounding rectangle to a small central region.
    grid.bounding_rectangle_set(0.15 * WIDTH, 0.85 * WIDTH, 0.15 * HEIGHT, 0.85 * HEIGHT);

    // Bigests first.
    for (text, area) in sorted_iter.clone().take(n_big) {
        grid.place_block(text.clone(), *area);
    }

    // Open up placement to the full surface.
    grid.bounding_rectangle_clear();

    for (text, area) in sorted_iter.skip(n_big) {
        grid.place_block(text.clone(), *area);
    }

    for b in grid.blocks {
        render_block(&b);
    }
    println!("</g>");
    println!("</svg>");

    // let word = random_word::gen(Lang::En);
}
