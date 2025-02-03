use crate::mishap::Mishap;
use anyhow::anyhow;
use axum::response::IntoResponse;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Response},
    routing::{get, put},
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
    // Database of dogs:
    dogs: HashMap<String, Dog>,

    // Holds id of currently selected dog
    // I'm following the example: I do not like this state management
    // (regardless that I've implemented it as global state, single user!)
    selected_id: Option<Uuid>,
}

impl DogDB {
    fn new() -> DogDB {
        let comet = Dog::new("Comet", "Whippet");
        let oscar = Dog::new("Oscar", "German Shorthaired Pointer");

        DogDB {
            dogs: HashMap::from([(comet.id.clone(), comet), (oscar.id.clone(), oscar)]),
            selected_id: None,
        }
    }

    fn dogs(&self) -> Vec<Dog> {
        self.dogs.values().cloned().collect()
    }

    fn find(&self, id: &Uuid) -> Option<&Dog> {
        self.dogs.values().find(|dog| dog.id == id.to_string())
    }

    fn select(&mut self, id: &Uuid) {
        self.selected_id = Some(*id);
    }

    fn deselect(&mut self) {
        self.selected_id = None;
    }

    fn insert(&mut self, dog: Dog) {
        self.deselect();
        self.dogs.insert(dog.id.clone(), dog);
    }

    fn delete(&mut self, id: Uuid) {
        self.deselect();
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
        .route("/dogs/form", get(stateful_form))
        .route("/dogs/select/{id}", put(select_dog))
        .route("/dogs/deselect", put(deselect))
        .route("/dogs/table-rows", get(table_rows))
        .route("/dogs/{id}", put(update_dog).delete(delete_dog))
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
    let markup = dog_row(&dog, None);
    Ok(markup.into_response())
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

async fn stateful_form(State(state): State<SharedState>) -> Result<Response, Mishap> {
    let db = state
        .read()
        .map_err(|_| Mishap(anyhow!("Read lock fail")))?;

    if let Some(id) = db.selected_id {
        let dog = db.find(&id).ok_or(Mishap(anyhow!("No such dog")))?;
        Ok(dog_form(dog).into_response())
    } else {
        Ok(blank_dog_form().into_response())
    }
}

async fn select_dog(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, Mishap> {
    let mut db = state
        .write()
        .map_err(|_| Mishap(anyhow!("Select write lock fail")))?;

    db.select(&id);

    Response::builder()
        .header("HX-Trigger", "selection-change")
        .status(200)
        .body(Body::empty())
        .map_err(|e| Mishap(anyhow!(e)))
}

async fn deselect(State(state): State<SharedState>) -> Result<Response, Mishap> {
    let mut db = state
        .write()
        .map_err(|_| Mishap(anyhow!("Select write lock fail")))?;

    db.deselect();

    Response::builder()
        .header("HX-Trigger", "selection-change")
        .status(200)
        .body(Body::empty())
        .map_err(|e| Mishap(anyhow!(e)))
}

async fn update_dog(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Form(form): Form<NewDog>,
) -> Result<Response, Mishap> {
    let mut db = state
        .write()
        .map_err(|_| Mishap(anyhow!("Write lock fail")))?;

    let dog = db.find(&id).ok_or(Mishap(anyhow!("No such dog")))?;

    let mut dog = dog.clone();
    dog.name = form.name;
    dog.breed = form.breed;

    db.insert(dog.clone());

    let row_html: String = dog_row(&dog, Some(true)).into();

    Response::builder()
        .header("HX-Trigger", "selection-change")
        .status(200)
        .body(Body::from(row_html))
        .map_err(|e| Mishap(anyhow!(e)))
}

async fn table_rows(State(state): State<SharedState>) -> Result<Response, Mishap> {
    let dogdb = state
        .read()
        .map_err(|_| Mishap(anyhow!("Read lock fail")))?;

    let frags: Vec<String> = dogdb
        .dogs()
        .iter()
        .map(|dog| dog_row(dog, None))
        .map(|m| m.into_string())
        .collect();

    Ok(frags.concat().into_response())
}

//
// HTML serialization
//

fn blank_dog_form() -> Markup {
    html! {
        form
            hx-disabled-elt="#submit-btn"
            hx-post="/dogs"
            hx-target="tbody"
            hx-swap="afterbegin" {
                div {
                    label for="name" { "Name" }
                    input id="name" name="name" required size="30" type="text" {}
                }
                div {
                    label for="breed" { "Breed" }
                    input  id="breed" name="breed" required size="30" type="text" {}
                }
                div class="buttons" {
                    button id="submit-btn" { "Add" }
                }
            }
    }
}

fn dog_form(dog: &Dog) -> Markup {
    let dog_url = format!("/dogs/{}", dog.id);

    html! {
        form
            hx-disabled-elt="#submit-btn"
            hx-put=(dog_url) {
                div {
                    label for="name" { "Name" }
                    input id="name" name="name" required size="30" type="text" value=(dog.name) {}
                }
                div {
                    label for="breed" { "Breed" }
                    input  id="breed" name="breed" required size="30" type="text" value=(dog.breed) {}
                }
                div class="buttons" {
                    button id="submit-btn" { "Update" }
                    button hx-put="/dogs/deselect" hx-swap="none" type="buttin" { "Cancel" }
                }

            }
    }
}

fn dog_row(dog: &Dog, swap_oob: Option<bool>) -> Markup {
    let dog_url = format!("/dogs/{}", dog.id);
    let dog_edit_url = format!("/dogs/select/{}", dog.id);

    // Our IDs are UUIDs, which can start with a number, but that would not be a valid CSS selector
    let row_id = format!("row-{}", dog.id);

    html! {
        tr class="on-hover" id=(row_id)  hx-swap-oob=[swap_oob] {
            td { (dog.name)  }
            td { (dog.breed) }
            td class="buttons" {
                button
                    class="show-on-hover"
                    hx-delete=(dog_url)
                    hx-confirm="Are you sure?"
                    hx-target="closest tr"
                    hx-swap="outerHTML"
                    type="button"
                    { "x" }
                button
                    class="show-on-hover"
                    hx-put=(dog_edit_url)
                    hx-swap="none"
                    type="button"
                    { "Edit" }
            }
        }
    }
}
