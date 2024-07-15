use leptos::create_signal;
use leptos::ReadSignal;
use leptos::WriteSignal;
use word_map::block::Blocks;

use crate::color_map::ColorMap;

/// State required by all pages.
#[derive(Clone)]
pub(crate) struct AppState {
  // Values returned form the server
  pub(crate) blocks: (ReadSignal<Blocks>, WriteSignal<Blocks>),
  // The colors assigned to the text/weight blocks (expressed as hex codes).
  pub(crate) color_signal: (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>),
  // Converts size of block to a color.
  pub(crate) color_map_signal: (ReadSignal<ColorMap>, WriteSignal<ColorMap>),
  // scale factor overall size of text blocks.
  pub(crate) scale_signal: (ReadSignal<u16>, WriteSignal<u16>),
  /// List of (test,weight) pairs each separated by spaces.
  pub(crate) text_weights_signal: (ReadSignal<String>, WriteSignal<String>),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
          blocks: create_signal::<Blocks>(Blocks::default()),
          color_signal: create_signal::<Vec<String>>(vec![]),
          color_map_signal: create_signal(ColorMap::new([5f64, 55f64], colorous::PLASMA)),
          scale_signal: create_signal(10_u16),
          text_weights_signal: create_signal(String::default()),
        }
    }
}
