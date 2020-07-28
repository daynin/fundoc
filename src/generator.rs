use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::config;
use crate::parser;

#[derive(Debug)]
struct Document {
    title: String,
    file_name: String,
    content: String,
}

const DEFAULT_DOCS_PATH: &str = "./docs";

fn to_markdown(document: &Document) -> String {
    format!("# {}\n{}", document.title, document.content)
}

fn update_dir(path: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path).ok();
    fs::create_dir_all(path)
}

fn merge_docs(
    articles: Vec<parser::Article>,
    repository_host: Option<String>,
) -> HashMap<String, Document> {
    let mut documentation: HashMap<String, Document> = HashMap::new();
    let repository_host = &repository_host;

    for article in articles {
        let file_name = article.topic.to_lowercase();
        let file_name = file_name.replace(" ", "_");
        let key = article.topic.clone();

        let document = documentation.entry(key).or_insert(Document {
            title: article.topic.clone(),
            file_name: format!("{}.md", file_name),
            content: "".to_string(),
        });

        let link = match repository_host {
            Some(host) => format!("[[~]]({}{})", host, article.path),
            None => "".to_string(),
        };

        document.content = format!(
            "{}\n{}\n{}\n",
            document.content,
            article.content.clone(),
            link
        );
    }

    documentation
}

fn write_doc(document: &Document, docs_path: &str) {
    let file_name = document.file_name.as_str();

    match File::create(format!("{}/{}", docs_path, file_name)) {
        Ok(mut file) => match file.write_all(to_markdown(document).as_bytes()) {
            Ok(_) => println!("\"{}\" is created", file_name),
            Err(_) => println!("Cannot write file {}", file_name),
        },
        Err(e) => println!("{:?}", e),
    }
}

pub fn generate_docs(articles: Vec<parser::Article>, config: config::Config) {
    println!("Start documentation generating...");

    let docs_path = config
        .docs_folder
        .or_else(|| Some(DEFAULT_DOCS_PATH.to_string()))
        .unwrap();
    let documentation = merge_docs(articles, config.repository_host);

    update_dir(&docs_path).expect("Cannot create the documentation folder");

    for key in documentation.keys() {
        let document = documentation.get(key);

        match document {
            Some(document) => write_doc(document, &docs_path),
            None => println!("Cannot find document"),
        }
    }

    println!("Done!");
}
