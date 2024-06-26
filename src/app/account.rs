use leptos::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};

use crate::error_template::{AppError, ErrorTemplate};

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
async fn get_account(name: String) -> Result<Account, ServerFnError<AppError>> {
    use crate::ssr::db;
    logging::log!("get_account: name={}", &name);

    let pool = db().await.unwrap();
    Ok(sqlx::query_as!(
        Account,
        "SELECT \"in_game_name\", \"region\" as \"region: _\", \"tag\" FROM account WHERE in_game_name=$1",
        name
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| AppError::InternalServerError)?
    .ok_or(AppError::AccountNotFound)?)
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
            <Suspense fallback=|| {view! {"Loading profile..."}}>
                // handles the error from the resource
                <ErrorBoundary fallback=|errors| {
                    view! {
                        <ErrorTemplate errors=errors/>
                    }
                }>
                    {move ||{
                        account.get().map(|account|{
                            account.map_err(|e| {
                                match e {
                                    ServerFnError::WrappedServerError(e) => e,
                                    _ => AppError::InternalServerError,
                                }
                            }).map(|account| {
                                view!{
                                    <p>"Account is: "{account.in_game_name} {account.region.to_str()} {account.tag}</p>
                                }
                            })
                        })
                    }}
                </ErrorBoundary>
            </Suspense>
        </section>
    }
}
