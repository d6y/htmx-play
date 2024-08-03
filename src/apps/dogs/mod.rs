use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use maud::{html, Markup};

use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid;

#[derive(Clone, Debug)]
struct Dog {
    id: String,
    name: String,
    breed: String,
}

impl Dog {
    fn new(name: &str, breed: &str) -> Dog {
        let id = uuid::Uuid::now_v7();
        Dog {
            id: id.to_string(),
            name: name.to_string(),
            breed: breed.to_string(),
        }
    }
}

//
// Fake in-memory dog database
//

type DogTable = HashMap<String, Dog>;
type AppState = Arc<Mutex<DogTable>>;

static DOG_DB: Lazy<AppState> = Lazy::new(|| {
    let comet = Dog::new("Comet", "Whippet");
    let oscar = Dog::new("Oscar", "German Shorthaired Pointer");

    let doggies = HashMap::from([(comet.id.clone(), comet), (oscar.id.clone(), oscar)]);
    Arc::new(Mutex::new(doggies))
});

fn with_db<T, F>(f: F) -> T
where
    F: FnOnce(&mut DogTable) -> T,
{
    let mut lock = DOG_DB.lock().unwrap();
    f(&mut lock)
}

// fn db_add_dog(dog: Dog) {
// let mut dogs = DOG_DB.lock().unwrap();
// let _maybe_preexisting_dog = dogs.insert(dog.id.clone(), dog);
// }

// fn num_dogs() -> usize {
// let dogs = DOG_DB.lock().unwrap();
// dogs.len()
// }

// fn seed_db_if_empty() {
// }

pub fn routes() -> Router {
    Router::new()
        .route("/dogs", get(index))
        .route("/dogs/table-rows", get(table_rows))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/dogs.html")).into_response()
}

fn dog_row(dog: &Dog) -> Markup {
    html! {
            tr class="on-hover" {
                td { (dog.name)  }
                td { (dog.breed) }
                td class="buttons" {
                    button
                        class="show-on-hover"
                        hx-delete=(dog.id)
                        hx-confirm="Are you sure?"
                        hx-target="closest tr"
                        hx-swap="delete" { "x" }

                }

        }
    }
}

async fn table_rows() -> Response {
    let dogs: Vec<Dog> = with_db(|dogs| dogs.values().cloned().collect());
    let frags: Vec<String> = dogs.iter().map(dog_row).map(|m| m.into_string()).collect();
    frags.concat().into_response()
}
