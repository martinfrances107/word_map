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
                        // log!("args {:#?}", args);
                        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                        let blocks_string: String = invoke("update", args).await.as_string().unwrap();
                        // log!("update_word_list() rx string blocks {:#?}", blocks_string);
                        let received_blocks: Blocks = serde_json::from_str(&blocks_string).unwrap();
                        // log!("update_word_list() rx blocks {:#?}", received_blocks);
                        blocks_set.set(received_blocks.0);

                    }
                }

                class="h-full"
                type="range"
                step="1"
                value="5"
                min="10"
                max="300"
            />
        </form>
    }
}
