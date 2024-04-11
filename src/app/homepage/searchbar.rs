use leptos::{component, view, IntoView};

#[component]
pub fn SearchBar() -> impl IntoView {
    view! {
        <div class="searchbar">
            <form action="/search" method="get">
                <img src="search-glass.svg" alt=""/>
                <input
                    type="text"
                    name="account"
                    placeholder="Search for an account"
                />
            </form>
        </div>
    }
}