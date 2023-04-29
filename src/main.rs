use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{scope, Json, ServiceConfig},
    Result,
};
use shuttle_actix_web::ShuttleActixWeb;
use std::path::PathBuf;

use shared::helpers::calculations::calculate_score;
use shared::models::{CompetitorInfo, Score};

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[post("")]
async fn calculate_results(competitor_info: Json<CompetitorInfo>) -> Result<Json<Score>> {
    let results = calculate_score(competitor_info.into_inner());
    Ok(Json(results))
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive();
        cfg.service(hello_world)
            .service(scope("/score").wrap(cors).service(calculate_results))
            .service(
                actix_files::Files::new("/", static_folder)
                    .show_files_listing()
                    .index_file("index.html"),
            );
    };
    Ok(config.into())
}
