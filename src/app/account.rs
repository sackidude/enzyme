use leptos::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};

use crate::error_template::ErrorTemplate;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
enum Region {
    EUW,
    EUNE,
    KR,
    NA,
}

impl Region {
    fn to_str(&self) -> String {
        match self {
            Region::EUW => "EUW",
            Region::EUNE => "EUNE",
            Region::KR => "KR",
            Region::NA => "NA",
        }
        .into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Account {
    in_game_name: String,
    region: Region,
    tag: String, // Maybe this should be something different as it can only be 4 char long
}

#[server]
async fn get_account(name: String) -> Result<Option<Account>, ServerFnError> {
    use crate::ssr::db;
    logging::log!("get_account: name={}", &name);

    let pool = db().await.unwrap();
    Ok(sqlx::query_as!(
        Account,
        "SELECT \"in_game_name\", \"region\" as \"region: _\", \"tag\" FROM account WHERE in_game_name=$1",
        name
    )
    .fetch_optional(&pool)
    .await?)
}

#[component]
fn LoadingProfile() -> impl IntoView {
    view! { <p>"Loading User"</p> }
}

#[component]
fn ErrorProfile() -> impl IntoView {
    view! { <p>"Something went wrong."</p> }
}

#[component]
pub fn AccountPage() -> impl IntoView {
    let params = use_query_map();

    let account = create_resource(
        move || params.get().get("name").cloned().unwrap_or_default(),
        move |account_name| async move { get_account(account_name).await },
    );
    view! {
        <section class="account-viewer">
            // handles the loading
            <Suspense fallback=LoadingProfile>
                // handles the error from the resource
                <ErrorBoundary fallback=|errors| {
                    view! {
                        // FIXME!: This doesn't work results in the following error when activated:
                        // Errors: []
                        // thread '<unnamed>' panicked at src/error_template.rs:53:39:
                        // index out of bounds: the len is 0 but the index is 0
                        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
                        // 
                        // I think it has to do with the error being ServerFnError and not this project error::Error
                        <ErrorTemplate errors=errors/>
                    }
                }>
                    {move || {
                        account
                            .get()
                            .map(move |account| {
                                account
                                    .map(move |account| {
                                        match account {
                                            None => view! { <p>"Couldn't find that account"</p> },
                                            Some(account) => {
                                                view! {
                                                    // the resource has a result and successful call from the server fn
                                                    <p>
                                                        "Account viewer: " {account.in_game_name}
                                                        {account.region.to_str()} {account.tag}
                                                    </p>
                                                }
                                            }
                                        }
                                    })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </section>
    }
}
