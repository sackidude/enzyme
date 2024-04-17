use std::marker::PhantomData;

use leptos::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Region {
    EUW,
    EUNE,
    KR,
    NA,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Account {
    in_game_name: String,
    region: Region,
    tag: String, // Maybe this should be something different as it can only be 4 char long
}

#[server(GetAccount)]
async fn get_account(name: String) -> Result<Account, ServerFnError> {
    logging::log!("get_account: name={}", &name);
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(Account {
        in_game_name: name,
        region: Region::EUW,
        tag: "spec".into(),
    })
}

#[component]
fn LoadingProfile() -> impl IntoView {
    view! { <p>"Loading User"</p> }
}

#[component]
fn ErrorProfile() -> impl IntoView {
    view! {
        <p>
            "Something went wrong, probably: Couldn't find that account, please try again."
        </p>
    }
}

#[component]
pub fn AccountPage() -> impl IntoView {
    let params = use_query_map();
    let account_name =
        move || params.with(|params| params.get("name").cloned().unwrap_or_default());
    // let account_name = params.get("name").cloned();

    let account = create_resource(|| (), |_| async { get_account("test".into()).await });
    view! {
        <section class="account-viewer">
            // handles the loading
            <Suspense fallback=LoadingProfile>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! {
                        // FIXME: Simplify this, ie dont wrap in view macro and ignore _.
                        <ErrorProfile/>
                    }
                }>
                    {move || {
                        account
                            .get()
                            .map(move |account| {
                                account
                                    .map(move |account| {
                                        view! {
                                            // the resource has a result and successful call from the server fn
                                            <p>
                                                "User result filled in server and client: "
                                                {account.in_game_name}
                                            </p>
                                        }
                                    })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </section>
    }
}
