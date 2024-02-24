use crate::models::document;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

/**
 * ドキュメント一覧を取得する
 */
pub async fn get_list() -> impl Responder {
    match document::get_documents() {
        Ok(documents) => HttpResponse::Ok().json(documents),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/**
 * ドキュメントを作成する
 */
pub async fn create() -> impl Responder {
    match document::create_document() {
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
