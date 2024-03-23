use crate::models::config::{get_linkedoc_dir, read_config, write_config};
use chrono::{DateTime, Local};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::str::FromStr;
use std::{
    fs::File,
    io::{self, Write},
};

/**
 * ドキュメント
 */
#[derive(Serialize, Deserialize)]
pub struct Document {
    /** ID */
    pub id: u32,
    pub path: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

/**
 * frontmatter の title, description, tags を抽出する
 */
fn extract_frontmatter(file_content: &str) -> Option<(String, String, Vec<String>)> {
    let re =  Regex::new(
        r#"\s*---\s*\n(?:title:\s*(.+)\n)?(?:description:\s*(.+)\n)?(?:tags:\s*\[(.+)\]\n)?\s*---\s*\n"#,
    ).expect("invalid regex");
    fn extract_field(caputures: &regex::Captures, index: usize) -> String {
        caputures
            .get(index)
            .map_or("", |m| m.as_str().trim())
            .to_string()
    }
    match re.captures(file_content) {
        Some(captures) => {
            let title = extract_field(&captures, 1);
            let description = extract_field(&captures, 2);
            let tags = extract_field(&captures, 3)
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            Some((title, description, tags))
        }
        None => None,
    }
}

#[test]
fn test_extract_frontmatter() {
    let file_content = r#"---
title: Document title
description: Document description
tags: [tag1, tag2]
---
"#;
    let document = extract_frontmatter(file_content).unwrap();
    assert_eq!(document.0, "Document title");
    assert_eq!(document.1, "Document description");
    assert_eq!(document.2, vec!["tag1".to_string(), "tag2".to_string()]);
}

/**
 * ドキュメントを取得する
 */
pub fn get_document(id: u32) -> Result<Document, io::Error> {
    let linkedoc_dir = get_linkedoc_dir()?;
    let file_name = format!("{}.md", id);
    let file_path = linkedoc_dir.join(file_name);
    let mut buf = String::new();
    let mut file = File::open(&file_path)?;
    file.read_to_string(&mut buf)?;
    let (title, description, tags) = match extract_frontmatter(&buf) {
        Some((title, description, tags)) => (title, description, tags),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid frontmatter in {}", file_path.display()),
            ))
        }
    };
    Ok(Document {
        id,
        path: "".to_string(),
        title,
        description,
        tags: tags.iter().map(|s| s.to_string()).collect(),
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

/**
 * ドキュメント一覧を取得する
 */
pub fn get_documents() -> Result<Vec<Document>, io::Error> {
    let linkedoc_dir = get_linkedoc_dir()?;
    let mut documents = vec![];
    // ".md" ファイルのみを documents に詰める
    for entry in linkedoc_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let mut buf = String::new();
            let mut file = File::open(&path)?;
            file.read_to_string(&mut buf)?;
            let (title, description, tags) = match extract_frontmatter(&buf) {
                Some((title, description, tags)) => (title, description, tags),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid frontmatter in {}", path.display()),
                    ))
                }
            };
            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".md");
            let document = Document {
                id: u32::from_str(file_name).unwrap(),
                path: "".to_string(),
                title,
                description,
                tags: tags.iter().map(|s| s.to_string()).collect(),
                created_at: Local::now(),
                updated_at: Local::now(),
            };
            documents.push(document);
        }
    }
    // ファイル名を昇順にソートする
    documents.sort_by(|a, b| {
        let a_num = a.id;
        let b_num = b.id;
        a_num.cmp(&b_num)
    });
    Ok(documents)
}

/**
 * ドキュメントを作成する
 */
pub fn create_document(
    path: &str,
    title: &str,
    description: &str,
    tags: &[String],
) -> Result<(), io::Error> {
    println!("create_document{}, {}, {}", path, title, description);

    let mut config = read_config()?;
    let new_file_name = format!("{}.md", config.latest_document_id + 1);
    let new_file_path = get_linkedoc_dir()?.join(new_file_name);

    // ファイルにメタデータを書き込む
    let mut file = File::create(new_file_path)?;
    let content = format!(
        "---\ntitle: {}\ndescription: {}\ntags: [{}]\n---\n",
        title,
        description,
        tags.join(", ")
    );
    file.write_all(content.as_bytes())?;

    config.latest_document_id += 1;
    write_config(&config)?;
    Ok(())
}
