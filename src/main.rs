use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use time::{macros::date, Date};
use uuid::Uuid;

pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_date: Date,
    pub stack: Vec<String>,
}

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

    people.insert(person.id, person);

    let app = Router::new()
        .route("/pessoas", get(get_person))
        .route("/pessoas", post(create_person))
        .route("/pessoas/:id", get(get_person_by_id))
        .route("/contagem-pessoas", get(count_person));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_person() -> impl IntoResponse {
    "pessoas"
}

async fn get_person_by_id() -> impl IntoResponse {
    "pessoa por id"
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::CREATED, "pessoa criada")
}

async fn count_person() -> impl IntoResponse {
    "contagem de pessoas"
}
