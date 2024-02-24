mod controllers;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Hello {
    message: String,
}

async fn hello() -> Result<impl Responder> {
    let hello = Hello {
        message: "Hello, world!".to_string(),
    };
    Ok(web::Json(hello))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"]);
        App::new()
            .wrap(cors)
            .route("/hello", web::get().to(hello))
            .configure(controllers::document::config)
            .configure(controllers::initialize::config)
            .service(
                actix_files::Files::new("/", "../client/out")
                    .index_file("index.html")
                    .show_files_listing(),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
