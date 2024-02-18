use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use std::{fs, io, path::PathBuf};

/**
 * "<HOME>/.linkedoc" のパスを取得する
 */
fn get_linkedoc_dir() -> Result<PathBuf, io::Error> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Home directory not found",
            ))
        }
    };
    Ok(home_dir.join("linkedoc"))
}

/**
 * 初期化が完了しているかどうか
 */
pub fn is_initialized() -> Result<bool, io::Error> {
    if get_linkedoc_dir()?.exists() {
        return Ok(true);
    }
    Ok(false)
}

/**
 * 初期化を行う
 */
pub async fn initialize() -> impl Responder {
    let linkedoc_dir = get_linkedoc_dir().unwrap();
    match is_initialized() {
        Ok(true) => HttpResponse::Ok().json("Already initialized"),
        Ok(false) => match fs::create_dir(linkedoc_dir) {
            Ok(_) => HttpResponse::Ok().json("Initialized"),
            Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/**
 * ルーティング設定
 */
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/initialize").route("", web::post().to(initialize)));
}
