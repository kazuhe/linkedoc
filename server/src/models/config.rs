use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{fs, io, path::PathBuf};
use std::{fs::File, io::Write};

/**
 * "<HOME>/linkedoc" ディレクトリのパスを取得する
 */
pub fn get_linkedoc_dir() -> Result<PathBuf, io::Error> {
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
pub struct Config {
    /** 最新のドキュメント ID */
    pub latest_document_id: i32,
}

/**
 * 設定ファイルのパスを取得する
 */
pub fn get_config_path() -> Result<PathBuf, io::Error> {
    Ok(get_linkedoc_dir()?.join("config.json"))
}

/**
 * 設定ファイルを書き込む
 */
pub fn write_config(config: &Config) -> Result<(), io::Error> {
    let config_path = get_config_path()?;
    let mut file = File::create(config_path)?;
    let config_json = serde_json::to_string_pretty(config)?;
    file.write_all(config_json.as_bytes())?;
    Ok(())
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
    write_config(&config)?;
    Ok(())
}

/**
 * 設定ファイルを読み込む
 */
pub fn read_config() -> Result<Config, io::Error> {
    let config_path = get_config_path()?;
    let mut file = File::open(config_path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);
    let config = serde_json::from_str(&buf)?;
    Ok(config)
}
