use axum::{
    http::Request,
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

    let routes = Router::new()
        .route("/", get(root))
        .route("/version", get(version))
        .merge(apps::dogs::routes())
        .merge(apps::oob::routes())
        .merge(apps::trigger::routes())
        .merge(apps::lazy::routes())
        .merge(apps::validate::routes())
        .merge(apps::pagination::routes());

    let app = Router::new().merge(assets).merge(routes);

    // During developemtn we want live-reload, but not of the htmx snippets

    #[cfg(debug_assertions)]
    fn not_htmx<Body>(req: &Request<Body>) -> bool {
        !req.headers().contains_key("hx-request")
    }

    #[cfg(debug_assertions)]
    let app = app.layer(tower_livereload::LiveReloadLayer::new().request_predicate(not_htmx));

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
