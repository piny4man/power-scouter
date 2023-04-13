use std::path::PathBuf;
use actix_web::{get, post, web::{ServiceConfig, Json, scope}, Result};
use shuttle_actix_web::ShuttleActixWeb;

use shared::models::{CompetitorInfo, Score};
use shared::helpers::calculations::calculate_score;

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
        cfg
            .service(hello_world)
            .service(
                scope("/score")
                    .service(calculate_results)
            )
            .service(
                actix_files::Files::new("/", static_folder)
                    .show_files_listing()
                    .index_file("index.html"),
            );
    };
    Ok(config.into())
}
