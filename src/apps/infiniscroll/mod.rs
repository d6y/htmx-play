use crate::components::image::Image;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use maud::html;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/infiniscroll", get(index))
        .route("/infiniscroll/image-rows", get(images))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/infiniscroll.html")).into_response()
}

const ROWS_PER_PAGE: usize = 5;

#[derive(Deserialize)]
struct Params {
    page: Option<usize>,
}

async fn images(params: Query<Params>) -> Response {
    let page_number = params.page.unwrap_or(1);

    let start_index = (page_number - 1) * ROWS_PER_PAGE;
    let next_index = start_index + ROWS_PER_PAGE;

    let images: Vec<Image> = Image::make(start_index..next_index);

    let is_last = |i| i == images.len() - 1;
    let next_page = format!("/infiniscroll/image-rows?page={}", page_number + 1);

    html! {
        @for (i, image) in images.iter().enumerate() {
            @if is_last(i) {
                tr hx-trigger="revealed"
                   hx-get=(next_page)
                   hx-indicator=".htmx-indicator"
                   hx-swap="afterend"  {
                    td { (image.name) }
                    td { img src=(image.url) width="320" height="240" {} }
                }
            }
            @else {
                tr  {
                    td { (image.name) }
                    td { img src=(image.url) {} }
                }
            }
        }
    }
    .into_response()
}
