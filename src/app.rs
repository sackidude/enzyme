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
                    <Route path="login" view=Login/>
                </Routes>
            </main>
            <footer>
                <Footer />
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
                    <a href="/login" class="login">"Login"</a>
                </li>
                <li>
                    <a href="/signup" class="signup">"Sign up"</a>
                </li>
            </ul>

        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
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

// Sign in related
#[component]
fn Login() -> impl IntoView {
    view! {
        <div class="login">
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
                        <input type="password" name="password" required/>
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
