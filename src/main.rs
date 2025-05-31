mod state;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Router, extract::State, routing::get};
use state::Db;

const LISTEN_ADDRESS: &str = "127.0.0.1:8088";

#[tokio::main]
async fn main() {
    // Add a logger
    tracing_subscriber::fmt::init();

    // Add a shared state
    let shared_state = Arc::new(Mutex::new(HashMap::<i8, String>::new()));

    // "Seed data"
    {
        // Q: How do we drop the lock and allow another reader/writer?
        let mut db = shared_state.lock().unwrap();
        db.insert(1, "Hello from db!".to_string());
    }

    // build app and attach shared state
    let app = Router::new().route("/", get(root)).with_state(shared_state);

    // Create our listener
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS)
        .await
        .expect("Unable to bind");

    // Run it!
    axum::serve(listener, app).await.expect("Axum error");
}

// Q: Why did return type change?
async fn root(State(state): State<Db>) -> String {
    if let Ok(hashmap) = state.lock() {
        format!("{:#?}", hashmap.values())
    } else {
        "Hello, SPRUG!".to_string()
    }
}
