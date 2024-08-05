use crate::mishap::Mishap;
use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{delete, get},
    Form, Router,
};
use maud::{html, Markup};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

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
        let id = Uuid::now_v7();
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

    fn delete(&mut self, id: Uuid) {
        let _before_delete = self.dogs.remove(&id.to_string());
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
        .route("/dogs", get(index).post(add_dog))
        .route("/dogs/table-rows", get(table_rows))
        .route("/dogs/:id", delete(delete_dog))
        .with_state(Arc::new(RwLock::new(db)))
}

async fn index() -> Response {
    Html(include_str!("../../../templates/dogs.html")).into_response()
}

async fn add_dog(
    State(state): State<SharedState>,
    Form(form): Form<NewDog>,
) -> Result<Response, Mishap> {
    let dog = Dog::new(&form.name, &form.breed);

    let mut db = state
        .write()
        .map_err(|_| Mishap(anyhow!("Write lock fail")))?;

    db.insert(dog.clone());
    Ok(dog_row(&dog).into_response())
}

async fn delete_dog(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, Mishap> {
    let mut db = state
        .write()
        .map_err(|_| Mishap(anyhow!("Write lock fail")))?;

    db.delete(id);
    Ok(StatusCode::OK.into_response())
}

async fn table_rows(State(state): State<SharedState>) -> Result<Response, Mishap> {
    let dogdb = state
        .read()
        .map_err(|_| Mishap(anyhow!("Read lock fail")))?;

    let frags: Vec<String> = dogdb
        .dogs()
        .iter()
        .map(dog_row)
        .map(|m| m.into_string())
        .collect();

    Ok(frags.concat().into_response())
}

//
// HTML serialization
//

fn dog_row(dog: &Dog) -> Markup {
    let dog_url = format!("/dogs/{}", dog.id);

    html! {
        tr class="on-hover" {
            td { (dog.name)  }
            td { (dog.breed) }
            td class="buttons" {
                button
                    class="show-on-hover"
                    hx-delete=(dog_url)
                    hx-confirm="Are you sure?"
                    hx-target="closest tr"
                    hx-swap="delete"
                    { "x" }
            }
        }
    }
}
