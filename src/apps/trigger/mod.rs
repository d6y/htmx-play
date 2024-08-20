use axum::{
    body::Body,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde_json::json;

pub fn routes() -> Router {
    Router::new()
        .route("/trigger", get(index))
        .route("/trigger/event-with-no-data", get(event_with_no_data))
        .route("/trigger/event-with-string", get(event_with_string))
        .route("/trigger/event-with-object", get(event_with_object))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/trigger.html")).into_response()
}

async fn event_with_no_data() -> Response {
    Response::builder()
        .status(200)
        .header("HX-Trigger", "event1")
        .body(Body::from("dispatched event 1"))
        .unwrap()
}

async fn event_with_string() -> Response {
    // This is delivered to the front-emd event handler for "event2" as
    // a CustomEvent with "detail" of an Object with "valie" of "some string".

    let payload = json!( {
        "event2": "some string",
    } );

    Response::builder()
        .status(200)
        .header("HX-Trigger", payload.to_string())
        .body(Body::empty())
        .unwrap()
}

async fn event_with_object() -> Response {
    let payload = json!( {
        "event3": {
            "foo": 1,
            "bar": 2,
        }
    } );

    Response::builder()
        .status(200)
        .header("HX-Trigger", payload.to_string())
        .body(Body::empty())
        .unwrap()
}
