use axum::{
    Router,
    routing::{get, post},
};
mod profilemodule;
mod schema;
mod usermodule;
mod utils;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nugget_sync_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = utils::db::establish_connection();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/register", post(usermodule::register_user))
        .route("/login", post(usermodule::login_user))
        .route(
            "/profiles",
            post(profilemodule::create_profile).get(profilemodule::get_profiles),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("NuggetSync Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
