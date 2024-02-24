use crate::models::config::{get_linkedoc_dir, read_config, write_config};
use std::{fs::File, io};

/**
 * ドキュメント一覧を取得する
 */
pub fn get_documents() -> Result<Vec<String>, io::Error> {
    let linkedoc_dir = get_linkedoc_dir()?;
    let mut documents = vec![];
    // ".md" ファイルのみを documents に詰める
    for entry in linkedoc_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            documents.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }
    // ファイル名を昇順にソートする
    documents.sort_by(|a, b| {
        let a_num: i32 = a.trim_end_matches(".md").parse().unwrap_or(0);
        let b_num: i32 = b.trim_end_matches(".md").parse().unwrap_or(0);
        a_num.cmp(&b_num)
    });
    Ok(documents)
}

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
