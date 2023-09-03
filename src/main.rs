use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pessoas", get(get_person))
        .route("/pessoas", post(create_person))
        .route("/pessoas/:id", get(get_person_by_id)).
        route( "/contagem-pessoas", get(count_person));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}   

async fn get_person() -> impl IntoResponse {
    "pessoas"
}

async fn create_person() -> impl IntoResponse {
    "pessoa criada"
}

async fn get_person_by_id() -> impl IntoResponse {
    "pessoa por id"
}

async fn count_person() -> impl IntoResponse {
    "contagem de pessoas"
}

