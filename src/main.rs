use axum::{Router, routing::get};

const LISTEN_ADDRESS: &str = "127.0.0.1:8088";

#[tokio::main]
async fn main() {
    // Add a logger
    tracing_subscriber::fmt::init();

    // build app
    let app = Router::new().route("/", get(root));

    // Create our listener
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS)
        .await
        .expect("Unable to bind");

    // Run it!
    axum::serve(listener, app).await.expect("Axum error");
}

async fn root() -> &'static str {
    "Hello, SPRUG!"
}
