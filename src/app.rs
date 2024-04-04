use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/enzyme.css"/>

        // sets the document title
        <Title text="Enzyme"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <header>
                <Navigation/>
            </header>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="signin" view=SignIn/>
                </Routes>
            </main>
            <footer>
                <h2>"Enzyme league journal"</h2>
            </footer>

        </Router>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <nav>
            <a href="/" aria-label="homepage">
                <img src="/logo.svg" alt=""/>
            </a>

            <ul>
                <li>
                    <a href="/signin">"Sign in"</a>
                </li>
                <li>
                    <a href="/signup">"Sign up"</a>
                </li>
            </ul>

        </nav>
    }
}

// Homepage related
#[component]
fn HomePage() -> impl IntoView {
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
                <img src="search-glass.svg" alt="search for account"/>
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
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
            </div>
            <div class="role">
                <h3>"Jungle"</h3>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
            </div>
            <div class="role">
                <h3>"Middle"</h3>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
            </div>
            <div class="role">
                <h3>"Bottom"</h3>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
            </div>
            <div class="role">
                <h3>"Support"</h3>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
                <p>"Potent#EUW"</p>
            </div>
        </div>
    }
}

// Sign in related
#[component]
fn SignIn() -> impl IntoView {
    view! {
        <div class="signin">
            <img src="logo.svg" alt=""/>
            <h1>"Sign in to Enzyme"</h1>
            <form class="inputbox" action="/login" method="POST">
                <label for="username">
                    <b>"Username"</b> <br/>
                    <input type="text" name="username" required/>
                </label>
                <div class="password-wrapper">
                    <label for="password">
                        <b> "Password"</b> <br/>
                        <input type="text" name="password" required/>
                    </label>
                    <a href="/forgotpassword">"Forgot password?"</a>
                </div>
                <input type="submit" value="Sign in"/>
            </form>

            <div class="create-account inputbox">
                <p>"New to Enzyme?"</p>
                <a href="/signup">"Create a profile"</a>
            </div>
        </div>
    }
}
