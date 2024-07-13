use leptos::create_signal;
use leptos::ReadSignal;
use leptos::WriteSignal;

use crate::color_map::ColorMap;

/// State required by all pages.
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) scale_signal: (ReadSignal<u16>, WriteSignal<u16>),
    pub(crate) color_map_signal: (ReadSignal<ColorMap>, WriteSignal<ColorMap>),
    // List of (test,weight) pairs each separated by spaces.
    pub(crate) text_weights_signal: (ReadSignal<String>, WriteSignal<String>),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            scale_signal: create_signal(10_u16),
            color_map_signal: create_signal(ColorMap::new([5f64, 55f64], colorous::PLASMA)),
            text_weights_signal: create_signal(String::default()),
        }
    }
}
