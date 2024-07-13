use leptos::component;
use leptos::view;
use leptos::IntoView;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Zoom and ColorScale
#[component]
pub fn ScaleBar() -> impl IntoView {
    use leptos::event_target_value;
    use leptos::use_context;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen_futures::spawn_local;
    use word_map::block::Blocks;

    use crate::app_state::AppState;
    use crate::UpdateArgs;

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
                    ev.prevent_default();
                    spawn_local(async move {
                        if let Ok(scale) = event_target_value(&ev).parse::<u16>() {
                            app_state.scale_signal.1.set(scale);
                            let tw = app_state.text_weights_signal.0.get_untracked();
                            let args = to_value(
                                    &UpdateArgs {
                                        scale: scale as f32,
                                        tw: &tw,
                                    },
                                )
                                .unwrap();
                            let blocks_string: String = invoke("update", args)
                                .await
                                .as_string()
                                .unwrap();
                        }
                    });
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
