use leptos::component;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn SideControls() -> impl IntoView {
    use crate::components::gradient_bar::GradientBar;
    view! {
        <div class="border-solid border-2 rounded-lg h-[600px] p-2">
            <GradientBar/>
        </div>
    }
}
