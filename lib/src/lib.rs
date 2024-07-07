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
