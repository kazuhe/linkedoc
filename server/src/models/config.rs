use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use std::{fs::File, io::Write};

/**
 * "<HOME>/linkedoc" ディレクトリのパスを取得する
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
 * 設定ファイル
 */
#[derive(Serialize, Deserialize)]
struct Config {
    /** 最新のドキュメント ID */
    latest_document_id: i32,
}

/**
 * 設定ファイルのパスを取得する
 */
pub fn get_config_path() -> Result<PathBuf, io::Error> {
    Ok(get_linkedoc_dir()?.join("config.json"))
}

/**
* 設定ファイルを作成する
*/
pub fn create_config() -> Result<(), io::Error> {
    // "<HOME>/linkedoc" ディレクトリが存在しない場合は作成する
    let linkedoc_dir = get_linkedoc_dir()?;
    if !linkedoc_dir.exists() {
        fs::create_dir(linkedoc_dir)?;
    }
    let config = Config {
        latest_document_id: 0,
    };
    let config_path = get_config_path()?;
    let mut file = File::create(config_path)?;
    let config_json = serde_json::to_string_pretty(&config)?;
    file.write_all(config_json.as_bytes())?;
    Ok(())
}
