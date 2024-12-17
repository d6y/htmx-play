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
        .route("/pagination", get(index))
        .route("/pagination/image-rows", get(images))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/pagination.html")).into_response()
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

    let back_url = format!("/pagination/image-rows?page={}", 1.max(page_number - 1));
    let forward_url = format!("/pagination/image-rows?page={}", (page_number + 1));

    html! {
        table id="image-table" {
            tr {
                th { "Name" }
                th { "Image" }
            }
            {
                @for image in images {
                    tr {
                        td {  (image.name) }
                        td { img src=(image.url) {} }
                    }
                }
            }
        }
       span id="pagination-buttons" hx-swap-oob="true" hx-indicator=".htmx-indicator" hx-target="#image-table" {
            button disabled[(page_number == 1)] hx-get=(back_url) { "Previous" }
            button hx-get=(forward_url) { "Next" }
        }
    }
    .into_response()
}
