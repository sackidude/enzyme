use leptos::{component, view, IntoView};

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="intro">
            <img src="logo.svg" alt=""/>
            <h1>"Enzyme"</h1>
            <p>
                "Accelerate your learning with reviews and note taking, the most proven
                learning method."
            </p>
        </div>
    }
}