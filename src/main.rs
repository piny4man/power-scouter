use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    web::{self, scope, Json, ServiceConfig},
    Result,
};
use shuttle_actix_web::ShuttleActixWeb;

use shared::helpers::calculations::calculate_score;
use shared::models::{CompetitorInfo, Score};

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn calculate_results(competitor_info: Json<CompetitorInfo>) -> Result<Json<Score>> {
    let results = calculate_score(competitor_info.into_inner());
    Ok(Json(results))
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive();
        cfg.service(
            scope("/api")
                .wrap(cors)
                .route("hello", web::get().to(hello_world))
                .route("score", web::post().to(calculate_results)),
        )
        .service(Files::new("/", "static").index_file("index.html"));
    };
    Ok(config.into())
}
