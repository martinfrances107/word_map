extern crate word_map;

// use word_map::block::Block;
use rand::Rng;
use word_map::grid::Grid;

static WIDTH: f32 = 800f32;
static HEIGHT: f32 = 600f32;

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
              --white: #f3ffff;
              --red: hsl(0, 100%, 50%);
              }}

              svg{{
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
    println!("<g font-family=\"Courier\">");
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
        println!("{}", b)
    }
    println!("</g>");
    println!("</svg>");

    // let word = random_word::gen(Lang::En);
}
