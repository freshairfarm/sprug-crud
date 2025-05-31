mod models;
mod state;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
};
use state::Db;

const LISTEN_ADDRESS: &str = "127.0.0.1:8088";

#[tokio::main]
async fn main() {
    // Add a logger
    tracing_subscriber::fmt::init();

    // Add a shared state
    let shared_state = Arc::new(Mutex::new(HashMap::<i8, models::Todo>::new()));

    // build app and attach shared state
    let app = Router::new()
        .route("/", get(root))
        .route("/todo", get(get_todos).post(create_todo))
        .route(
            "/todo/{id}",
            post(update_todo).get(get_todo).delete(delete_todo),
        )
        .with_state(shared_state);

    // Create our listener
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS)
        .await
        .expect("Unable to bind");

    // Run it!
    axum::serve(listener, app).await.expect("Axum error");
}
async fn delete_todo(State(state): State<Db>, Path(id): Path<i8>) {
    todo!()
}
async fn get_todo(
    State(state): State<Db>,
    Path(id): Path<i8>,
) -> Result<Json<models::Todo>, StatusCode> {
    let db = state.lock().unwrap();
    if let Some(todo) = db.get(&id) {
        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
async fn update_todo(
    State(state): State<Db>,
    Path(id): Path<i8>,
    Json(payload): Json<models::UpdateTodo>,
) -> Result<Json<models::Todo>, StatusCode> {
    let mut db = state.lock().unwrap();

    if let Some(todo) = db.get_mut(&id) {
        todo.content = payload.content;
        if let Some(todo_status) = payload.completed {
            todo.completed = todo_status;
        }
        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
async fn create_todo(
    State(state): State<Db>,
    Json(payload): Json<models::CreateTodo>,
) -> Json<models::Todo> {
    let mut db = state.lock().unwrap();
    let new_id = db.keys().max().map(|id| id + 1).unwrap_or(1);
    let todo = models::Todo {
        id: new_id,
        content: payload.content,
        completed: false,
    };

    db.insert(new_id, todo.clone());
    Json(todo)
}
async fn get_todos(State(state): State<Db>) -> Json<Vec<models::Todo>> {
    if let Ok(hashmap) = state.lock() {
        let values: Vec<models::Todo> = hashmap.values().cloned().collect();
        Json(values)
    } else {
        Json(Vec::new())
    }
}
async fn root() -> &'static str {
    "Hello, SPRUG!"
}
