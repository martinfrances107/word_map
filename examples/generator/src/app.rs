use leptos::component;
use leptos::view;
use leptos::IntoView;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use word_map::block::Block;
use word_map::Orientation;
use word_map::Point2d;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdateArgs<'a> {
    pub(crate) scale: f32,
    // (text,weight) expressed as a string
    pub(crate) tw: &'a str,
}

fn render_block(b: &Block, text_fill: &String) -> impl IntoView {
    // rec width is not text width.
    let rec_width = b.top_right.x - b.bottom_left.x;
    // rec_height is not text height.
    let rec_height = b.bottom_left.y - b.top_right.y;

    let rect_x = b.bottom_left.x;
    let rect_y = b.bottom_left.y - rec_height;

    let text = match b.orientation {
        Orientation::Horizontal => {
            let transform = format!(
                "translate({}, {}) rotate(0)",
                b.bottom_left.x, b.bottom_left.y
            );

            view! {
                <text transform=transform fill=text_fill font-size=rec_height>
                    {b.text.clone()}
                </text>
            }
        }
        Orientation::Vertical90 => {
            // origin is top left
            let top_left = Point2d {
                x: b.top_right.x - rec_width,
                y: b.top_right.y,
            };
            let transform = format!("translate({}, {}) rotate(90)", top_left.x, top_left.y);
            view! {
                <text transform=transform fill=text_fill font-size=rec_width>
                    {b.text.clone()}
                </text>
            }
        }
        Orientation::Vertical270 => {
            // origin is bottom right
            let bottom_right = Point2d {
                x: b.bottom_left.x + rec_width,
                y: b.bottom_left.y,
            };
            let transform = format!(
                "translate({}, {}) rotate(270)",
                bottom_right.x, bottom_right.y
            );
            view! {
                <text transform=transform fill=text_fill font-size=rec_width>
                    {b.text.clone()}
                </text>
            }
        }
    };

    let rect_x = b.bottom_left.x;
    let rect_y = b.bottom_left.y - rec_height;
    let bl_cy = b.bottom_left.y - rec_height;
    view! {
        <rect x=rect_x y=rect_y width=rec_width height=rec_height></rect>
        <circle class="bl" cx=b.bottom_left.x cy=b.bottom_left.y r="2"></circle>
        <circle class="tr" cx=b.top_right.x cy=b.top_right.y r="2"></circle>
        {text}
    }
}

#[component]
pub fn App() -> impl IntoView {
    use leptos::leptos_dom::ev::SubmitEvent;
    use leptos::logging::log;
    use leptos::*;
    use rand::Rng;
    use random_word::Lang;

    use serde_wasm_bindgen::to_value;
    use word_map::block::Block;
    use word_map::block::Blocks;

    use crate::app_state::AppState;
    use crate::components::scale_bar::ScaleBar;
    use crate::components::side_controls::SideControls;

    let mut rng = rand::thread_rng();

    let app_state = AppState::default();
    provide_context(app_state.clone());

    // List of SVG elements representing the block, expressed as a single string.
    let (blocks, blocks_set) = create_signal::<Vec<Block>>(vec![]);

    let prepare_text_weights = move |ev| {
        let v = event_target_value(&ev);
        app_state.text_weights_signal.1.set(v);
    };

    let update_word_list = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let tw: String = app_state.text_weights_signal.0.get_untracked();
            if tw.is_empty() {
                return;
            }

            let args = to_value(&UpdateArgs {
                scale: app_state.scale_signal.0.get_untracked() as f32,
                tw: &tw,
            })
            .unwrap();

            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let blocks_string: String = invoke("update", args).await.as_string().unwrap();
            let received_blocks: Blocks = serde_json::from_str(&blocks_string).unwrap();
            blocks_set.set(received_blocks.0);
        });
    };

    // string literal here facilitates escaping of \{ and \}
    static CSS: &str = r#"
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
                        let color = app_state.color_signal.0.get();
                        let ibc = blocks.get().into_iter().enumerate().zip(color);
                        view! {
                            // ibc - index/block/color

                            <For
                                each=move || ibc.clone()
                                key=|((i, _block), _color)| { *i }
                                let:data
                            >

                                {
                                    let ((_, ref block), ref color) = data;
                                    render_block(block, color)
                                }

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
                        prop:value=app_state.text_weights_signal.0
                    />
                    <button class="w-fit" type="submit">
                        "Update"
                    </button>
                    <button
                        class="w-fit"
                        type="submit"
                        on:click=move |_| {
                            let mut text_weights = String::default();
                            let mut colors = Vec::with_capacity(100);
                            let color_map = app_state.color_map_signal.0.get();
                            for i in 0..100 {
                                let area = rng.gen_range(10u32..100u32);
                                let text = random_word::gen(Lang::En).to_uppercase();
                                colors.push(color_map.rgb(area as f64));
                                text_weights.push_str(&format!("{text},{area} "));
                            }
                            app_state.text_weights_signal.1.set(text_weights);
                            app_state.color_signal.1.set(colors);
                        }
                    >

                        "Random"
                    </button>

                </form>

            </div>

        </main>
    }
}
