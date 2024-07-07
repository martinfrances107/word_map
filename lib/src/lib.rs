use rand::{rngs::ThreadRng, Rng};

pub(crate) mod block;
pub mod grid;

#[derive(Clone, Debug)]
struct Point2d {
    x: f32,
    y: f32,
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    // A rotation of 90 clockwise
    // Text runs Downwards.
    Vertical90,
    // A rotation of 90 clockwise
    // Text runs Upwards.
    Vertical270,
}

impl Orientation {
    fn at_random(rng: &mut ThreadRng) -> Self {
        let i = rng.gen_range(0..3);
        if i == 0 {
            Orientation::Horizontal
        } else if i == 1 {
            Orientation::Vertical90
        } else {
            Orientation::Vertical270
        }
    }
}
