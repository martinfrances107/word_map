pub mod grid;
pub(crate) mod block;

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