use colorous::Gradient;
use leptos::component;
use leptos::event_target_value;
use leptos::view;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::WriteSignal;

use crate::components::gradient_bar::GradientBar;

/// Zoom and ColorScale
#[component]
pub fn SideControls() -> impl IntoView {
    view! {
        <div class="border-solid border-2 rounded-lg h-[600px] p-2">

            <GradientBar/>

        </div>
    }
}
