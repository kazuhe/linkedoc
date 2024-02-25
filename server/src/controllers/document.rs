use crate::models::document;
use actix_web::{
    web::{self, Json, ServiceConfig},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

/**
 * ドキュメント一覧を取得する
 */
pub async fn get_list() -> impl Responder {
    match document::get_documents() {
        Ok(documents) => HttpResponse::Ok().json(documents),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateDocumentRequest {
    path: String,
    title: String,
    description: String,
    tags: Vec<String>,
}

/**
 * ドキュメントを作成する
 */
pub async fn create(form: Json<CreateDocumentRequest>) -> impl Responder {
    match document::create_document(&form.path, &form.title, &form.description, &form.tags) {
        Ok(_) => HttpResponse::Ok().json("Document created"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/**
* ルーティング設定
*/
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/document")
            .route("", web::get().to(get_list))
            .route("", web::post().to(create)),
    );
}
