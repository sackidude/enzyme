use leptos::*;
use leptos_router::{use_params_map, Outlet};

pub fn wrapper() -> impl IntoView {
    view! {
        <p>"This is a wrapper"</p>
        <Outlet/>
        <p>"End of wrapper"</p>
    }
}

pub fn default() -> impl IntoView{
    view! { <h1>"Search for an account"</h1> }
}

pub fn account() -> impl IntoView {
    let params = use_params_map();
    let account = move || params.with(|params| params.get("Account").cloned().unwrap_or_default());

    view! { <h1>"This is account: " {account}</h1> }
}