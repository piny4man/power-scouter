use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use std::net::SocketAddr;

use shared::helpers::calculations::calculate_score;
use shared::models::{CompetitorInfo, Score};

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn calculate_results(Json(competitor_info): Json<CompetitorInfo>) -> Json<Score> {
    let results = calculate_score(competitor_info);
    Json(results)
}

#[tokio::main]
async fn main() {
    let frontend_static_files = ServeDir::new("../static");

    let app = Router::new()
        .route("/api/hello", get(hello_world))
        .route("/api/score", post(calculate_results))
        .nest_service("/", frontend_static_files);

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Starting server to {}", address);

    axum::serve(listener, app).await.unwrap();
}
