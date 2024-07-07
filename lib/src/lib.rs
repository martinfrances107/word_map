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
    Vertical,
}
