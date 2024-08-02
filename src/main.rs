use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let assets = Router::new().nest_service("/assets", ServeDir::new("assets"));

    let app = Router::new()
        .route("/", get(root))
        .merge(assets)
        .route("/version", get(version));

    #[cfg(debug_assertions)]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root(req: axum::http::Request<Body>) -> Response {
    ServeFile::new("templates/index.html")
        .oneshot(req)
        .await
        .into_response()
}

async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
