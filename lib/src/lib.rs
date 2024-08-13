#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! A library for packing text close together.
//!

use rand::{rngs::ThreadRng, Rng};
use serde::Deserialize;
use serde::Serialize;

extern crate leptos;
extern crate nom;
extern crate serde;

/// A block is a collection of characters with associated data.
pub mod block;

/// A collection of block data.
pub mod grid;

/// Primitive  representation of a point on the canvas.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point2d {
    /// x coordinate
    pub x: f32,
    /// y coordinate
    pub y: f32,
}

/// State of the text object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Orientation {
    /// No rotation.
    Horizontal,
    /// A rotation of 90 clockwise
    /// Text runs Downwards.
    Vertical90,
    /// A rotation of 90 clockwise
    /// Text runs Upwards.
    Vertical270,
}

impl Orientation {
    fn at_random(rng: &mut ThreadRng) -> Self {
        let i = rng.gen_range(0..3);
        if i == 0 {
            Self::Horizontal
        } else if i == 1 {
            Self::Vertical90
        } else {
            Self::Vertical270
        }
    }
}
