extern crate colorous;
extern crate word_map;

mod app;
mod app_state;
mod color_map;

mod components;
use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}
