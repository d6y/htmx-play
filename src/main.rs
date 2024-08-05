use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

mod apps;
mod mishap;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let assets = Router::new().nest_service("/assets", ServeDir::new("assets"));

    let app = Router::new()
        .route("/", get(root))
        .merge(assets)
        .merge(apps::dogs::routes())
        .route("/version", get(version));

    #[cfg(debug_assertions)]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Response {
    Html(include_str!("../templates/index.html")).into_response()
}

async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
