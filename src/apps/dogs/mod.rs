use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};
use maud::{html, Markup};

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid;

//
// Datastructure and a fake in-memory database
//

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

struct DogDB {
    dogs: HashMap<String, Dog>,
}

impl DogDB {
    fn new() -> DogDB {
        let comet = Dog::new("Comet", "Whippet");
        let oscar = Dog::new("Oscar", "German Shorthaired Pointer");

        DogDB {
            dogs: HashMap::from([(comet.id.clone(), comet), (oscar.id.clone(), oscar)]),
        }
    }

    fn dogs(&self) -> Vec<Dog> {
        self.dogs.values().cloned().collect()
    }

    fn insert(&mut self, dog: Dog) {
        self.dogs.insert(dog.id.clone(), dog);
    }
}

type SharedState = Arc<RwLock<DogDB>>;

//
// Forms, routes and route handers
//

#[derive(serde::Deserialize)]
struct NewDog {
    name: String,
    breed: String,
}

pub fn routes() -> Router {
    let db = DogDB::new();

    Router::new()
        .route("/dogs", get(index).post(add))
        .route("/dogs/table-rows", get(table_rows))
        .with_state(Arc::new(RwLock::new(db)))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/dogs.html")).into_response()
}

async fn add(State(state): State<SharedState>, Form(form): Form<NewDog>) -> Response {
    let dog = Dog::new(&form.name, &form.breed);
    state.write().unwrap().insert(dog.clone());
    dog_row(&dog).into_response()
}

async fn table_rows(State(state): State<SharedState>) -> Response {
    let frags: Vec<String> = state
        .read()
        .unwrap()
        .dogs()
        .iter()
        .map(dog_row)
        .map(|m| m.into_string())
        .collect();
    frags.concat().into_response()
}

//
// HTML serialization
//

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
                    hx-swap="delete"
                    { "x" }
            }
        }
    }
}
