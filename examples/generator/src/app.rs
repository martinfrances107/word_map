use leptos::component;
use leptos::IntoView;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct UpdateArgs<'a> {
    // (text,weight) expressed as a string
    tw: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    use leptos::leptos_dom::ev::SubmitEvent;
    use leptos::logging::log;
    use leptos::*;
    use serde_wasm_bindgen::to_value;
    use word_map::block::Block;
    use word_map::block::Blocks;
    use crate::app_state::AppState;
    use crate::components::scale_bar::ScaleBar;
    use crate::components::side_controls::SideControls;

    let app_state = AppState::default();
    provide_context(app_state.clone());

    // List of (test,weight) pairs each separated by spaces.
    let (text_weights, text_weights_set) = create_signal(String::new());
    // List of SVG elements representing the block, expressed as a single string.
    let (blocks, blocks_set) = create_signal::<Vec<Block>>(vec![]);

    let prepare_text_weights = move |ev| {
        let v = event_target_value(&ev);
        text_weights_set.set(v);
    };

    let update_word_list = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let tw: String = text_weights.get_untracked();
            if tw.is_empty() {
                return;
            }

            let args = to_value(&UpdateArgs { tw: &tw }).unwrap();

            log!("args {:#?}", args);
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let blocks_string: String = invoke("update", args).await.as_string().unwrap();
            log!("update_word_list() rx string blocks {:#?}", blocks_string);
            let received_blocks: Blocks = serde_json::from_str(&blocks_string).unwrap();
            log!("update_word_list() rx blocks {:#?}", received_blocks);
            blocks_set.set(received_blocks.0);
        });
    };

    // string literal here facilitates escaping of \{ and \}
    static CSS: &str= r#"
      #word_map {
      --prussianBlue: #003153;
      --white: hsl(232, 0%, 95%);
      --red: hsl(0, 100%, 50%);

      background-color: var(--prussianBlue);
      font-family: Courier;
      }

      /* bottom left of text block */
      .bl {
        fill: var(--red);
        stroke: None;
      }

      /* top right corner of text block */
      .tr {
        fill: var(--red);
        stroke: None;
      }

      #word_map rect {
        stroke: var(--white);
        fill: none
      }

      #word_map text {
        fill: var(--white);
        font-weight: bold;
      }
    "#;

    view! {
        <main class="flex flex-col gap-4 p-2">
            <div class="flex">
                <h1 class="text-5xl">WORD MAP</h1>
            </div>

            <div class="flex p-2">
                <svg
                    id="word_map"
                    class="border border-2 border-solid mr-2 rounded-lg"
                    width="800px"
                    height="600px"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <defs>
                        <style>{CSS}</style>
                    </defs>

                    {move || {
                        view! {
                            <For each=move || blocks.get() key=|block| { block.text.clone() } let:b>
                                {b.into_view()}
                            </For>
                        }
                    }}

                </svg>
                <SideControls/>
            </div>
            <div class="border-solid border-2 flex-none rounded-lg p-4">
                <ScaleBar/>
            </div>
            <div class="border-solid border-2 flex flex-col rounded-lg p-4">
                <h2 class="text-lg font-bold">Words</h2>
                <p>Enter a list of word/weight pairs</p>
                <p>A comma separated list of word,weight pairs</p>
                <p>A weight is a number between 1 and 10</p>
                <form class="flex flex-col p-2" on:submit=update_word_list>
                    <input
                        id="word-weight-input"
                        placeholder="apple,1 socks,10 house,5"
                        on:input=prepare_text_weights
                    />
                    <button class="w-fit" type="submit">
                        "Update"
                    </button>
                </form>

            </div>

        </main>
    }
}
