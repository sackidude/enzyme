use leptos::*;
use leptos_router::Form;

#[component]
pub fn SearchBar() -> impl IntoView {
    view! {
        <div class="searchbar">
            <Form method="GET" action="account">
                <img src="search-glass.svg" alt=""/>
                <input
                    type="text"
                    name="name"
                    placeholder="Search for an account"
                />
            </Form>
        </div>
    }
}
