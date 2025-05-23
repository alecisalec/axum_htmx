use axum::{
    http::Method, 
    Router,
};

use tower_http::cors::{Any, CorsLayer};

use super::practice_routes;


pub fn create_routes() -> Router {
   

    // cors middleware
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .merge(practice_routes::practice_routes())
        .layer(cors)
        
}