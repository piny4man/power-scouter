use std::path::PathBuf;

use actix_web::{get, web::ServiceConfig};
use shuttle_service::ShuttleActixWeb;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_service::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
        .service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index.html")
        );
    })
}
