use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, Json,
};
use time::{macros::date, Date};
use uuid::Uuid;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_date: Date,
    pub stack: Vec<String>,
}

type AppState = Arc<HashMap<Uuid, Person>>;

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();
    let id = Uuid::now_v7();

    let person = Person {
        id: id,
        name: "Gustavo".to_string(),
        nick: "swxtz".to_string(),
        birth_date: date!(2005 - 10 - 13),
        stack: vec!["Rust".to_string(), "Typescript".to_string()],
    };

    println!("{}", person.id);
    people.insert(person.id, person);

    let app_state = Arc::new(people);

    let app = Router::new()
        .route("/pessoas", get(get_person))
        .route("/pessoas", post(create_person))
        .route("/pessoas/:id", get(get_person_by_id))
        .route("/contagem-pessoas", get(count_person))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_person() -> impl IntoResponse {
    "pessoas"
}

async fn get_person_by_id(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {

    let person = people.get(&person_id);

    match person {
        Some(person) => Ok(Json(person.clone())),
        None => Err((StatusCode::NOT_FOUND, "pessoa não encontrada")),
    }
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::CREATED, "pessoa criada")
}

async fn count_person() -> impl IntoResponse {
    "contagem de pessoas"
}
