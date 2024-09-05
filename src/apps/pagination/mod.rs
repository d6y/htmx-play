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

struct Image {
    name: String,
    url: String,
}

async fn images(params: Query<Params>) -> Response {
    let page_number = params.page.unwrap_or(1);

    let start_index = (page_number - 1) * ROWS_PER_PAGE;
    let next_index = start_index + ROWS_PER_PAGE;

    let images: Vec<Image> = (start_index..next_index).map(make_image).collect();

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

fn make_image(i: usize) -> Image {
    let (r, g, b) = int_to_rgb(i);

    let bg_colour = format!("{:02X}{:02X}{:02X}", r, g, b);
    let fg_colour = format!("{:02X}{:02X}{:02X}", 255 - r, 255 - g, 255 - b);

    let name = format!("Image {}", i);

    let url = format!(
        "https://placehold.co/320x240/{}/{}?text=Image+number+{}",
        bg_colour, fg_colour, i
    );

    Image { name, url }
}

// This code below mostly from ChatGPT 4o

fn int_to_rgb(n: usize) -> (u8, u8, u8) {
    // Increment the hue by a fixed step to spread colors evenly
    let hue = (n as f64 * 137.508) % 360.0; // 137.508 is the golden angle for more uniform distribution
    let saturation = 0.7; // Keep saturation constant
    let lightness = 0.5; // Keep lightness constant

    hsl_to_rgb(hue, saturation, lightness)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

    let (r1, g1, b1) = match h_prime {
        h if (0.0..1.0).contains(&h) => (c, x, 0.0),
        h if (1.0..2.0).contains(&h) => (x, c, 0.0),
        h if (2.0..3.0).contains(&h) => (0.0, c, x),
        h if (3.0..4.0).contains(&h) => (0.0, x, c),
        h if (4.0..5.0).contains(&h) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let m = l - c / 2.0;
    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    (r, g, b)
}
