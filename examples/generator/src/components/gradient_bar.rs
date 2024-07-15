use leptos::component;
use leptos::view;
use leptos::IntoView;

use crate::app_state::AppState;

/// ColorScale
#[component]
pub fn GradientBar(#[prop(default = "PALLETTE")] title: &'static str) -> impl IntoView {
    use leptos::use_context;
    use leptos::CollectView;
    use leptos::SignalGet;

    let app_state = use_context::<AppState>().expect("GridentBar: Failed to retrieve state");

    view! {
        <div class="flex flex-col items-center">
            <p class="text-center flex-none">{title}</p>
            <div id="scale" class="border-solid radius-sm border-1 w-4 my-1 rounded-lg">
                <svg
                    style="width: 100%; height:500px;"
                    preserveAspectRatio="none"
                    viewBox="0 0 1 200"
                    version="1.1"
                    xmlns="http://www.w3.org/2000/svg\"
                >

                    {(0..100)
                        .map(|pos| {
                            view! {
                                <rect
                                    y=pos * 2
                                    width="1"
                                    height="3"
                                    fill=move || {
                                        app_state.color_map_signal.0.get().rgb(f64::from(pos))
                                    }
                                >
                                </rect>
                            }
                        })
                        .collect_view()}

                </svg>
            </div>
        </div>
    }
}
