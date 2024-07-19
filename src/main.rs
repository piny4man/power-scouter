mod components;
mod helpers;
mod models;
mod routes;

use axum::{
    extract::Json,
    response::{self, Html},
    routing::{get, post},
    Router,
};
use dioxus::prelude::*;
use tower_http::services::ServeDir;

use std::{net::SocketAddr, path::PathBuf};

use helpers::calculations::calculate_score;
use models::{CompetitorInfo, Score};
use routes::home::home_route;

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn calculate_results(Json(competitor_info): Json<CompetitorInfo>) -> Json<Score> {
    let results = calculate_score(competitor_info);
    Json(results)
}

async fn home_endpoint() -> response::Result<Html<String>> {
    let mut app = VirtualDom::new(home_route);
    let _ = app.rebuild();

    Ok(Html(dioxus_ssr::render(&app)))
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/api/hello", get(hello_world))
        .route("/api/score", post(calculate_results))
        .route("/", get(home_endpoint))
        .nest_service("/public", ServeDir::new(PathBuf::from("public")));

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Starting server to http://{}", address);

    axum::serve(listener, router).await.unwrap();
}
