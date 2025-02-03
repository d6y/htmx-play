use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use maud::{html, Markup};
use rand::seq::IndexedRandom;

pub fn routes() -> Router {
    Router::new()
        .route("/lazy", get(index))
        .route("/lazy/users", get(users))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/lazy.html")).into_response()
}

async fn users() -> Response {
    // Pretend this takes a while:
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let mut rng = rand::rng();
    let users = User::randos(&mut rng).take(8);

    html! {
        table {
            thead { (User::render_header()) }
            tbody { @for user in users { (User::render_row(user))   } }
        }
    }
    .into_response()
}

struct User {
    first: String,
    last: String,
    company: String,
    email: String,
}

impl User {
    fn render_header() -> Markup {
        html! {
            tr {
                th { "Given" }
                th { "Family" }
                th { "Email" }
                th { "Company" }
            }
        }
    }

    fn render_row(self) -> Markup {
        html! {
            tr {
                td { (self.first) }
                td { (self.last) }
                td { (self.email) }
                td { (self.company) }
            }
        }
    }
}

impl User {
    fn randos<R: rand::Rng>(rng: &mut R) -> impl Iterator<Item = User> + '_ {
        (0..).map(|_| Self::rando(rng))
    }

    fn rando<R: rand::Rng>(rng: &mut R) -> User {
        let first = Self::FIRST_NAMES.choose(rng).unwrap();
        let last = Self::LAST_NAMES.choose(rng).unwrap();

        let email = format!(
            "{}.{}@example.org",
            first.to_lowercase(),
            last.to_lowercase()
        );

        let company = format!(
            "{} {} Inc",
            Self::HYPE_TERMS.choose(rng).unwrap(),
            Self::TECH_TERMS.choose(rng).unwrap()
        );

        User {
            first: first.to_string(),
            last: last.to_string(),
            company,
            email,
        }
    }

    const FIRST_NAMES: [&'static str; 10] = [
        "Alex", "Sam", "Jordan", "Élise", "Taylor", "Morgan", "Joaquín", "Riley", "Quinn", "Avery",
    ];

    const LAST_NAMES: [&'static str; 10] = [
        "Elephant", "Penguin", "Octopus", "Kangaroo", "Platypus", "Koala", "Narwhal", "Axolotl",
        "Sloth", "Panda",
    ];

    const HYPE_TERMS: [&'static str; 10] = [
        "Mega", "Awesome", "Epic", "Rad", "Super", "Hyper", "Ultra", "Mighty", "Cool", "Turbo",
    ];

    const TECH_TERMS: [&'static str; 10] = [
        "Byte", "Code", "Data", "Quantum", "Cyber", "Nano", "Web", "Cloud", "Net", "Tech",
    ];
}
