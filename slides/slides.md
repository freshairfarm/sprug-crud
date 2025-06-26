---
title: Sprug Crud
sub_title: A simple todo API
author: Spokane Rust
---
 Thank You!
===
# Limelyte

Limelyte is a local software development company...
<Limelyte logo here?>
<!-- end_slide -->
 Thank You!
===
# Rust Foundation

Rust Foundation sponsored our... 
<Rust logo here?>
<!-- end_slide -->
Spokane Tech
===
<!-- end_slide -->
What's new in Rust?
===
<!-- end_slide -->
Goals
===

# Introduce Axum
<!-- speaker_note: Axum is a modern, ergonomic web framework in Rust -->

# Build a simple, real-world API 
<!-- speaker_note: | 
    
    Demonstrate shared state management
    Implement (mostly) CRUD
    Show practical API patterns
-->

# Get comfortable with Rust
<!-- speaker_note: | 

    Step through the project, distinct git branches
-->

<!-- end_slide -->
Step 1: Start the Project
===
```sh {1|2|3-5}+line_numbers
  cargo new sprug-crud
  cd sprug-crud
  cargo add axum dotenvy serde-json tracing-subscriber
  cargo add serde --features derive
  cargo add tokio --features full
```

We just:
* Created a new Rust project
* Added libraries for serializing/deserializing, async, error tracing, and a web framework

Now we can do a quick `cargo run` to verify everything works! 
<!--speaker_note: |
    This creates a new Rust project called sprug-crud.
    
    ----
    
    axum – the web framework we'll be using.

    dotenvy – allows us to load config from a .env file if needed later.

    serde-json – for working with JSON data.

    tracing-subscriber – gives us structured logs and good diagnostics out of the box.

    Adds Serde, which lets us serialize and deserialize Rust structs.

    We enable the derive feature so we can use #[derive(Serialize, Deserialize)].

    Tokio is the async runtime. Axum is built on top of it.

    The --features full flag gives us the full feature set, including TCP, macros, etc.

    ----
-->
<!-- end_slide -->
Step 2: Hello Axum!
===
# Adding error handling
```rust {8} +line_numbers
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

```

* `tracing_subcriber` gives us a nice log output
<!--speaker_note: |
    This sets up structured logging using the tracing ecosystem.

    replacement for println! in async apps — much more performant and structured.
-->
<!-- end_slide -->
Step 2: Hello Axum!
===
# Making it async

```rust {3-4|4,20|12-14,17} +line_numbers
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

```

* `#[tokio::main]` is a macro that sets up the runtime
* `async fn` makes our route handler compatible with `axum`'s async context
* We use `await` to bind and serve the app
<!-- speaker_note: |
    This is a macro provided by the tokio crate.

    It automatically sets up a Tokio runtime, which is required to run async functions.

    Without this, you can’t .await anything — the compiler will complain.

    It wraps the main function and ensures that all async code inside can execute.

    👉 Think of it as main() with batteries included.

    ----

    main is now async because we use .await when:

    Binding our TCP listener

    Starting the Axum server

    root is also async, which is required by Axum — all route handlers must be async fn.

    ----

    These are non-blocking operations. That means the runtime can continue doing other work (like handling other requests) while it waits.
-->
<!-- end_slide -->
Step 2: Hello Axum!
===
# Creating our first API endpoint & Handler
```rust {1-2|8-17|20-22} +line_numbers
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

```

* `const`: compile-time constants, gets embedded in the binary
* `async fn root()`: our `/` handler, defines what happens when a client calls `/`

This is the simplest example to start an async HTTP server that responds to `GET /`

<!--end_slide-->
Step 3: Shared State Setup
===
# Create a "database"
```rust
// src/state.rs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type Db = Arc<Mutex<HashMap<i8, String>>>;
```
* `Arc`: Atomic Ref Count (safe shared ownership)
* `Mutex`: Local wrapper that prevents race conditions
* `HashMap`: Simple in-memory store
<!-- speaker_note: |
    This is our actual store — it maps an ID to a string

    Right now, it’s basic: 1 => "do the thing"

    Later we’ll replace String with a proper Todo struct
    
    ----

    HTTP servers are multithreaded

    Two requests could hit our HashMap at the same time

    A Mutex ensures only one thread accesses the data at a time

    This prevents race conditions — so the data stays safe and consistent.

    ----

    Arc stands for Atomic Reference Count

    It lets us share ownership of the Mutex<HashMap> between threads

    Without Arc, we couldn’t move the state into multiple route handlers

    This combination — Arc<Mutex<T>> — is a classic Rust pattern for safe, shared, mutable state.

    ----

    We’re using a type alias so we don’t repeat that long Arc<Mutex<...>> everywhere

    Now we can just write Db in function signatures and it’s clear what it means
-->
<!--end_slide-->
Step 3: Shared State Setup
===
# Use our new database
```rust {1-7, 11-18|21-22,25-33} +line_numbers
mod state;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use axum::{Router, extract::State, routing::get};
use state::Db;
..
async fn main() {
..
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
..
}

// Q: Why did return type change?
async fn root(State(state): State<Db>) -> String {
    if let Ok(hashmap) = state.lock() {
        format!("{:#?}", hashmap.values())
    } else {
        "Hello, SPRUG!".to_string()
    }
}

```
* We seed the database with some sample data
* Now `root` pulls from our shared "database"
<!--end_slide-->
Step 4: GET /todos
===
# Create a new `Todo` struct
```rust {2|3-7} +line_numbers
// src/models.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i8,
    pub content: String,
    pub completed: bool,
}
```
# Step 4: GET /todos
* `Clone`: Needed for copying inside the handler
* `Debug`: Used for debug printing
* `Serialize/Deserialize`: JSON I/O

Super simple struct
<!--end_slide-->
Step 4: GET /todos
===
# Use `Todo` in our `Db`
```rust {6-7} +line_numbers
// src/state.rs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use crate::models::Todo;
pub type Db = Arc<Mutex<HashMap<i8, Todo>>>;
```
* We import our new `struct`
* Instead of using a simple `String`, we now use our `Todo` struct
<!--end_slide-->
Step 4: GET /todos
===
# Use `Todo` in our `app`
```rust {7|10-20|23-26} +line_numbers
mod models;
mod state;
use axum::{Json, Router, extract::State, routing::get};
...
async fn main() {
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
        .with_state(shared_state);
...
}

```
<!--end_slide-->
Step 4: GET /todo
===
# Modify our handlers

```rust +line_numbers
// src/main.rs
async fn get_todos(State(state): State<Db>) -> Json<Vec<models::Todo>> {
    if let Ok(hashmap) = state.lock() {
        let values: Vec<models::Todo> = hashmap.values().cloned().collect();
        Json(values)
    } else {
        Json(Vec::new())
    }
}
```
* Cloning allows non-blocking response construction
* We return JSON back to the client
<!--end_slide-->
Step 5: POST /todo
===
# Create a DTO
```rust +line_numbers
// src/models.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub content: String,
}
```
<!--end_slide-->
Step 5: POST /todo
===
# Handle it
```rust {7|12|13|16|17|18-22|24-25} +line_numbers
...
{
    // build app and attach shared state
    let app = Router::new()
        .route("/", get(root))
        .route("/todo", get(get_todos))
        .route("/todo", post(create_todo))
        .with_state(shared_state);
}
...
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
...
```
<!-- speaker_note: | 
    This is the POST /todo endpoint — it allows clients to create a new todo.

    State(state): State<Db>: Axum automatically extracts shared state for us (our in-memory DB).

    Json(payload): Json<CreateTodo>: Axum deserializes the request body into our CreateTodo struct.

    We lock the database (mutex) to get write access.

    A: panics if the mutex is poisoned — could be replaced with .expect(...) or better error handling in production.

    Build a new `todo`

    Finally, insert into DB and return the JSON
-->