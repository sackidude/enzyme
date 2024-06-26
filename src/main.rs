#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use dotenv::dotenv;
    use enzyme::app::*;
    use enzyme::fileserv::file_and_error_handler;
    use enzyme::ssr::db;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

<<<<<<< HEAD
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("{}", database_url);

    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("failect to connect to database");
=======
    // Some setup
    dotenv().ok();

    let pool = db().await.expect("couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("couldn't migrate database");
    logging::log!("migrated database");

>>>>>>> 14724b401e965bd42945e6a9881c35345da27560
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
