use leptos::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};

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
async fn get_account(name: String) -> Result<Account, ServerFnError> {
    use crate::ssr::db;
    logging::log!("get_account: name={}", &name);

    let pool = db().await.unwrap();
    // TODO!: Check if "unchecked" is necessary
    // I think so because region is enum
    let account = sqlx::query_as_unchecked!(
        Account,
        "SELECT \"in_game_name\", \"region\" as Region, \"tag\" FROM account WHERE in_game_name=$1",
        name
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(ServerFnError::new("couldn't find account"))?; // TODO!: Better error with statuscode.

    Ok(account)
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

    let account = create_resource(
        move || params.get().get("name").cloned().unwrap_or_default(),
        move |account_name| async move { get_account(account_name).await },
    );
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
                                                {account.region.to_str()}
                                                {account.tag}
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
