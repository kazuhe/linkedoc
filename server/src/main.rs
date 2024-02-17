use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/hello", web::get().to(hello)).service(
            actix_files::Files::new("/", "../client/out")
                .index_file("index.html")
                .show_files_listing(),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
