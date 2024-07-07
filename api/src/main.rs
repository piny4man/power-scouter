use actix_files::Files;
use actix_web::{
    web::{self, scope, Json},
    App, HttpServer, Result,
};

use shared::helpers::calculations::calculate_score;
use shared::models::{CompetitorInfo, Score};

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn calculate_results(competitor_info: Json<CompetitorInfo>) -> Result<Json<Score>> {
    let results = calculate_score(competitor_info.into_inner());
    Ok(Json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                scope("/api")
                    .route("hello", web::get().to(hello_world))
                    .route("score", web::post().to(calculate_results)),
            )
            .service(Files::new("/", "../static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
