use leptos::component;
use leptos::view;
use leptos::IntoView;

/// Zoom and ColorScale
#[component]
pub fn ScaleBar() -> impl IntoView {
    use leptos::event_target_value;
    use leptos::use_context;
    use leptos::SignalGet;
    use leptos::SignalSet;

    use crate::app_state::AppState;

    let app_state = use_context::<AppState>().expect("ScaleBar: Failed to retrieve state");

    view! {
        <form class="flex gap-4 items-center">
            <label class="font-bold" for="zoom">
                "ZOOM"
            </label>
            <p>{move || app_state.scale_signal.0.get()}</p>
            <input
                id="zoom"
                on:change=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<u16>() {
                        app_state.scale_signal.1.set(val);
                    }
                }

                class="h-full"
                type="range"
                step="1"
                value="5"
                min="300"
                max="800"
            />
        </form>
    }
}
