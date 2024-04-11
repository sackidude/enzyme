mod intro;
mod searchbar;
mod pros;

use leptos::{component, view, IntoView};

use crate::app::homepage::{intro::Intro, pros::Pros, searchbar::SearchBar};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="homepage">
            <Intro/>
            <SearchBar/>
            <Pros/>
        </div>
    }
}
