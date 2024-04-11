use leptos::{component, view, IntoView};

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav>
            <a href="/" aria-label="homepage">
                <img src="/logo.svg" alt=""/>
            </a>

            <ul>
                <li>
                    <a href="/login" class="login">"Login"</a>
                </li>
                <li>
                    <a href="/signup" class="signup">"Sign up"</a>
                </li>
            </ul>

        </nav>
    }
}