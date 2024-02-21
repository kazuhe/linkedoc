use crate::models::config;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use std::io;

/**
* 初期化が完了しているかどうか
*/
pub fn is_initialized() -> Result<bool, io::Error> {
    let config_path = config::get_config_path()?;
    if config_path.exists() {
        return Ok(true);
    }
    Ok(false)
}

/**
* 初期化を行う
*/
pub async fn initialize() -> impl Responder {
    match is_initialized() {
        Ok(true) => HttpResponse::Ok().json("Already initialized"),
        Ok(false) => {
            if let Err(err) = config::create_linkedoc_dir() {
                return HttpResponse::InternalServerError().json(err.to_string());
            }
            if let Err(err) = config::create_config() {
                return HttpResponse::InternalServerError().json(err.to_string());
            }
            HttpResponse::Ok().json("Initialized")
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/**
* ルーティング設定
*/
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/initialize").route("", web::post().to(initialize)));
}
