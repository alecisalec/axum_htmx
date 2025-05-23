use axum::{
    routing::get, 
    Router,
};


pub fn practice_routes() -> Router {
    Router::new()
        .route("/", get(hello))
    // Add more routes here
}

async fn hello() -> String {
    "Hello, World!".to_owned()
}