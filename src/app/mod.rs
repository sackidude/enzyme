mod account;
mod footer;
mod homepage;
mod login;
mod navigation;

use crate::{
    app::{
        account::AccountPage, footer::Footer, homepage::HomePage, login::Login,
        navigation::Navigation,
    },
    error_template::{AppError, ErrorTemplate},
};
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
                    <Route path="account" view=AccountPage/>
                </Routes>
            </main>
            <footer>
                <Footer/>
            </footer>
        </Router>
    }
}
