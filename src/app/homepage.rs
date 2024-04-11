use leptos::{component, view, IntoView};

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

#[component]
fn Intro() -> impl IntoView {
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

#[component]
fn SearchBar() -> impl IntoView {
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

#[component]
fn Pros() -> impl IntoView {
    view! {
        <div class="pros">
            <h2>"Check out and review some pros"</h2>
            <div class="role">
                <h3>"Top"</h3>
                <ul>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                </ul>
            </div>
            <div class="role">
                <h3>"Jungle"</h3>
                <ul>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
            </ul>

            </div>
            <div class="role">
                <h3>"Middle"</h3>
                <ul>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
                <li>"Potent#EUW"</li>
            </ul>

            </div>
            <div class="role">
                <h3>"Bottom"</h3>
                <ul>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                </ul>
            </div>
            <div class="role">
                <h3>"Support"</h3>
                <ul>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                    <li>"Potent#EUW"</li>
                </ul>
            </div>
        </div>
    }
}