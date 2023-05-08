use actix_cors::Cors;
use actix_web::{
    web::{self, scope, Data, Json, ServiceConfig},
    HttpRequest, Result,
};
use shuttle_actix_web::ShuttleActixWeb;
use std::{path::PathBuf, sync::Mutex};

use shared::helpers::calculations::calculate_score;
use shared::models::{CompetitorInfo, Score};

struct AppState {
    scores: Mutex<Vec<Score>>,
}

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn calculate_results(
    req: HttpRequest,
    competitor_info: Json<CompetitorInfo>,
) -> Result<Json<Score>> {
    let results = calculate_score(competitor_info.into_inner());
    let state = req
        .app_data::<Data<AppState>>()
        .expect("app_data is empty!");
    state.scores.lock().unwrap().push(results.clone());
    Ok(Json(results))
}

async fn scores_history(req: HttpRequest) -> Result<Json<Vec<Score>>> {
    let state = req
        .app_data::<Data<AppState>>()
        .expect("app_data is empty!");
    let scores = state.scores.lock().unwrap();
    Ok(Json(scores.clone()))
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    let scores = Data::new(AppState {
        scores: Mutex::new(Vec::new()),
    });
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive();
        cfg.service(
            scope("/")
                .app_data(scores.clone())
                .wrap(cors)
                .route("hello", web::get().to(hello_world))
                .route("score", web::post().to(calculate_results))
                .route("history", web::get().to(scores_history)),
        )
        .service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index.html"),
        );
    };
    Ok(config.into())
}
