use axum::{
    extract::Path,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use axum_session::{Session, SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use maud::{html, Markup};

const BUTTONS: [&str; 4] = ["Blitzen", "Prancer", "Dancer", "Vixen"];

pub async fn routes() -> Router {
    let session_config = SessionConfig::default().with_table_name("toggle");
    let store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    Router::new()
        .route("/toggle", get(index))
        .route("/toggle/buttons", get(buttons))
        .route("/toggle/toggle/{name}", get(toggle))
        .layer(SessionLayer::new(store))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/toggle.html")).into_response()
}

async fn buttons() -> Response {
    html! {
        @for name in BUTTONS {
            (button(name.to_string(), false, false))
        }
    }
    .into_response()
}

/// We render a pain old button, styling if it is current selected.
///
/// When it comes to updating a button, we use an out-of-band swap.
/// Now, the target for the swap must exist for this to work.
/// As a result the initial render (in `buttons`, above) we omit
/// the OOB swap.
///
/// * `name` the button name (HTML ID)
/// * `selected` true if the button is to be rendered as selected
/// * `swap` true to swap an existing element; false to create it.
fn button(name: String, selected: bool, swap: bool) -> Markup {
    let toggle = format!("/toggle/toggle/{}", name);

    // Idiom for a fixed Some(String) value or None
    let class = Some("selected").filter(|_| selected);
    let swap = Some("true").filter(|_| swap);
    html! {
        button class=[class] hx-get=(toggle) id=(name) hx-swap-oob=[swap] {
            (name)
        }
    }
}

async fn toggle(Path(name): Path<String>, session: Session<SessionNullPool>) -> Response {
    if let Some(current_selection) = session.get::<String>("selected_button_name") {
        // Something is already selected
        if current_selection == name {
            // So that click was to unselect the current option
            session.clear();
            button(name, false, true).into_response()
        } else {
            // Unselect current and select a different button
            session.set("selected_button_name", name.to_owned());
            html! {
                (button(current_selection, false, true))
                (button(name, true, true))
            }
            .into_response()
        }
    } else {
        // Nothing is current selected, so just select the item:
        session.set("selected_button_name", name.to_owned());
        button(name, true, true).into_response()
    }
}
