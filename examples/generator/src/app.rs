use leptos::component;
use leptos::IntoView;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    use crate::app_state::AppState;
    use crate::components::scale_bar::ScaleBar;
    use crate::components::side_controls::SideControls;
    use leptos::leptos_dom::ev::SubmitEvent;
    use leptos::*;

    let app_state = AppState::default();
    provide_context(app_state.clone());

    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_word_list = move |ev| {
        // let v = event_target_value(&ev);
        // set_name.set(v);
    };

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }

    //         let args = to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! {
        <main class="flex flex-col gap-4 p-2">
            <div class="flex">
                <h1 class="text-5xl">WORD MAP</h1>
                /* TODO nav here */
            </div>
            /*
              By design this is the row that sets the width of the screen.

              All other rows are relative to this wdith!!!
            */
            <div class="flex p-2">
              <svg class="border border-2 border-solid mr-2 rounded-lg" width="800px" height="600px" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
              </svg>
              <SideControls/>
            </div>
            <div class="border-solid border-2 flex-none rounded-lg p-4">
              <ScaleBar />
            </div>
            <div class="border-solid border-2 flex flex-col rounded-lg p-4">
              <h2 class="text-lg font-bold">Words</h2>
              <p>Enter a list of word/weight pairs</p>
              <p>A comma separated list of word,weight pairs</p>
              <p>A weight is a number between 1 and 10</p>
              <form class="flex flex-col p-2">
                <input
                    id="word-weight-input"
                    placeholder="apple,1 socks,10 house,5"
                    on:input=update_word_list
                />
                <button class="w-fit" type="submit">"Update"</button>
              </form>
            </div>

        </main>
    }
}
