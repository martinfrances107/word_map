extern crate word_map;

// use word_map::block::Block;
use rand::Rng;
use word_map::{block::Block, grid::Grid, Orientation, Point2d};

static WIDTH: f32 = 800f32;
static HEIGHT: f32 = 600f32;

fn render_block(b: &Block) {
    // rec width is not text width.
    let rec_width = b.top_right.x - b.bottom_left.x;
    // rec_height is not text height.
    let rec_height = b.bottom_left.y - b.top_right.y;

    // println!(
    //     "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
    //     b.bottom_left.x, b.bottom_left.y - rec_height, rec_width, rec_height
    // );

    // Want dots ontop of rectangle.
    println!(
        "<circle class=\"bl\" cx=\"{}\" cy=\"{}\" r=\"2\" />",
        b.bottom_left.x, b.bottom_left.y
    );
    println!(
        "<circle class=\"tr\" cx=\"{}\" cy=\"{}\" r=\"2\" />",
        b.top_right.x, b.top_right.y
    );

    match b.orientation {
        Orientation::Horizontal => {
            println!(
                "<text transform=\"translate({}, {}) rotate(0)\" font-size=\"{}\" >{}</text>",
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
              "<text transform=\"translate({}, {}) rotate(90)\" fill=\"\" font-size=\"{}\" >{}</text>",
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
                "<text transform=\"translate({}, {}) rotate(270)\" font-size=\"{}\" >{}</text>",
                bottom_right.x, bottom_right.y, rec_width, b.text
            )
        }
    }
}

fn main() {
    use random_word::Lang;
    let mut rng = rand::thread_rng();

    let mut grid = Grid::new(WIDTH, HEIGHT);

    // Limit to bounding rectangle
    // grid.bounding_rectangle_set(
    //     WIDTH / 3_f32,
    //     2_f32 * WIDTH / 3_f32,
    //     HEIGHT / 3_f32,
    //     2_f32 * HEIGHT / 3_f32,
    // );

    println!("<?xml version=\"1.0\" standalone=\"no\"?><!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg version=\"1.1\"
      width=\"{WIDTH}\"
      height=\"{HEIGHT}\"
      xmlns=\"http://www.w3.org/2000/svg\"
    >");
    println!(
        r"<defs>
            <style><![CDATA[
              svg{{
              --prussianBlue: #003153;
              --white: hsl(232, 0%, 95%);
              --red: hsl(0, 100%, 50%);
              font-family: Courier;
              background-color: var(--prussianBlue);
              }}
              /* bottom left of text block */
              .bl {{
                fill: var(--red);
                stroke: None;
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
          </defs>"
    );
    // Assign a random number to a word selected at random
    for _ in 0..900 {
        // input range 1..10 ( no zero width )
        //
        // assuming a char width of 24px
        // maps to a screen area based on 24x24 squares
        let area = 24_f32 * 24_f32 * rng.gen_range(1_f32..10_f32);
        let text = random_word::gen(Lang::En).to_uppercase();
        grid.place_block(text, area);
    }

    for b in grid.blocks {
        render_block(&b);
    }
    println!(r"</svg>");

    // let word = random_word::gen(Lang::En);
}
