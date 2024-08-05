use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Thank you: https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs

pub struct Mishap(pub anyhow::Error);

impl IntoResponse for Mishap {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Mishap
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
