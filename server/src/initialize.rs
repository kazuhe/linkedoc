use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use std::{fs::File, io::Write};

/**
 * 設定ファイル
 */
#[derive(Serialize, Deserialize)]
struct Config {
    /** 最新のドキュメント ID */
    latest_document_id: i32,
}

/**
 * "<HOME>/linkedoc" のパスを取得する
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
 * 設定ファイルのパスを取得する
 */
fn get_config_path() -> Result<PathBuf, io::Error> {
    Ok(get_linkedoc_dir()?.join("config.json"))
}

/**
 * 設定ファイルを作成する
 */
fn create_config() -> Result<(), io::Error> {
    let config = Config {
        latest_document_id: 0,
    };
    let config_path = get_config_path()?;
    let mut file = File::create(config_path)?;
    let config_json = serde_json::to_string_pretty(&config)?;
    file.write_all(config_json.as_bytes())?;
    Ok(())
}

/**
 * 初期化が完了しているかどうか
 * 設定ファイルがあるかどうかで判断する
 */
pub fn is_initialized() -> Result<bool, io::Error> {
    let config_path = get_config_path()?;
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
            let linkedoc_dir = get_linkedoc_dir().unwrap();
            if !linkedoc_dir.exists() {
                if let Err(err) = fs::create_dir(linkedoc_dir) {
                    return HttpResponse::InternalServerError().json(err.to_string());
                }
            }
            if let Err(err) = create_config() {
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
