use rand::{rngs::ThreadRng, Rng};
use serde::Deserialize;
use serde::Serialize;

extern crate leptos;
extern crate nom;
extern crate serde;

pub mod block;
pub mod grid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Orientation {
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
