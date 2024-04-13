use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn Account() -> impl IntoView {
    let params = use_query_map();
    let account = move || params.with(|params| params.get("name").cloned().unwrap_or_default());

    view! { <h1>"This is account: " {account}</h1> }
}
