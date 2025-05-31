mod models;
mod state;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use state::Db;

const LISTEN_ADDRESS: &str = "127.0.0.1:8088";

#[tokio::main]
async fn main() {
    // Add a logger
    tracing_subscriber::fmt::init();

    // Add a shared state
    let shared_state = Arc::new(Mutex::new(HashMap::<i8, models::Todo>::new()));

    // "Seed data"
    {
        let mut db = shared_state.lock().unwrap();
        db.insert(
            1,
            models::Todo {
                id: 1,
                content: "Finish this API!".to_string(),
                completed: false,
            },
        );
    }

    // build app and attach shared state
    let app = Router::new()
        .route("/", get(root))
        .route("/todo", get(get_todos))
        .route("/todos", post(create_todo))
        .with_state(shared_state);

    // Create our listener
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS)
        .await
        .expect("Unable to bind");

    // Run it!
    axum::serve(listener, app).await.expect("Axum error");
}
async fn create_todo(
    State(state): State<Db>,
    Json(payload): Json<models::CreateTodo>,
) -> Json<models::Todo> {
    // Q: This is shorter, but what are the risks?
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
async fn root(State(state): State<Db>) -> String {
    if let Ok(hashmap) = state.lock() {
        format!("{:#?}", hashmap.values())
    } else {
        "Hello, SPRUG!".to_string()
    }
}
