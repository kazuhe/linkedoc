use crate::models::document::create_document;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

/**
 * ドキュメントを作成する
 */
pub async fn create() -> impl Responder {
    match create_document() {
        Ok(_) => HttpResponse::Ok().json("Document created"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/**
* ルーティング設定
*/
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/document").route("", web::post().to(create)));
}
