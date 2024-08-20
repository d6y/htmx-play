use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use maud::html;

pub fn routes() -> Router {
    Router::new()
        .route("/oob", get(index))
        .route("/oob/demo", get(demo))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/oob.html")).into_response()
}

async fn demo() -> Response {
    html! {
    div { "new 1" }
    div id="target2" hx-swap-oob="true" { "new 2" }
    div id="target2" hx-swap-oob="afterend" { "after 2" }
    div hx-swap-oob="innerHTML:#target3" { "new 3" }
    }
    .into_response()
}
