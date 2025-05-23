mod routes;

pub async fn run() {
    
  
    // Create routes with database pool
    let app = routes::route_controller::create_routes();
    
    // Run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}