use leptos::ServerFnError;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn db() -> Result<Pool<Postgres>, ServerFnError> {
    let url =
        std::env::var("DATABASE_URL").expect(".env variable `DATABASE_URL` couldn't be found.");
    Ok(sqlx::postgres::PgPoolOptions::new().connect(&url).await?)
}
