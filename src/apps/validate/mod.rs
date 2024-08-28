use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/validate", get(index))
        .route("/validate/email-validate", get(email_validate))
}

#[derive(Deserialize)]
struct Params {
    email: String,
}

async fn index() -> Response {
    Html(include_str!("../../../templates/validate.html")).into_response()
}

async fn email_validate(params: Query<Params>) -> Response {
    let msg = if params.email.trim().ends_with("example.org") {
        "Address in use"
    } else {
        ""
    };

    msg.into_response()
}
