use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="copyright">
            <h2>"Enzyme"</h2>
            <p>"© 2024"</p>
        </div>
        <div class="vertical-line"></div>
        <div class="links">
            <a href="/privacy">"Privacy"</a>
            <a href="/contact">"Contact"</a>
            <a href="/TOS">TOS</a>
            <a href="/about">"About"</a>
        </div>
    }
}