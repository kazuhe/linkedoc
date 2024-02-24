use crate::models::config::{get_linkedoc_dir, read_config, write_config};
use std::{fs::File, io};

/**
 * ドキュメントを作成する
 */
pub fn create_document() -> Result<(), io::Error> {
    let mut config = read_config()?;
    let new_file_name = format!("{}.md", config.latest_document_id + 1);
    let new_file_path = get_linkedoc_dir()?.join(new_file_name);
    File::create(new_file_path)?;
    config.latest_document_id += 1;
    write_config(&config)?;
    Ok(())
}
