use std::collections::HashMap;
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

fn to_markdown(document: &Document) -> String {
    format!("# {}\n{}", document.title, document.content)
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
        let key = file_name.clone();

        let document = documentation.entry(key).or_insert(Document {
            title: article.topic.clone(),
            file_name: format!("{}.md", file_name),
            content: "".to_string(),
        });

        let link = match repository_host {
            Some(host) => format!(
                "[[~]]({}{}#L{}-L{})",
                host, article.path, article.start_line, article.end_line
            ),
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
            Err(_) => println!("Cannot write a file: {}", file_name),
        },
        Err(e) => println!("{:?}", e),
    }
}

fn create_summary(documents: &HashMap<String, Document>) -> String {
    let mut content = String::from("# Summary\n\n");
    let mut keys = documents.keys().collect::<Vec<_>>();
    keys.sort();

    for key in keys {
        let document = documents.get(key);

        match document {
            Some(document) => {
                content += format!("* [{}](./{})\n", document.title, document.file_name).as_str()
            }
            None => println!("Cannot find document"),
        }
    }

    content
}

fn write_summary(documents: &HashMap<String, Document>, docs_path: &str, mdbook: bool) {
    match File::create(format!(
        "{}/{}.md",
        docs_path,
        if mdbook { "SUMMARY" } else { "README" }
    )) {
        Ok(mut file) => match file.write_all(create_summary(documents).as_bytes()) {
            Ok(_) => println!("Summary is created",),
            Err(_) => println!("Cannot create the summary file"),
        },
        Err(e) => println!("{:?}", e),
    }
}

pub fn generate_docs(articles: Vec<parser::Article>, config: config::Config) {
    let docs_path = config.docs_folder.unwrap();
    let documentation = merge_docs(articles, config.repository_host);

    write_summary(&documentation, &docs_path, config.mdbook.unwrap());

    for key in documentation.keys() {
        let document = documentation.get(key);

        match document {
            Some(document) => write_doc(document, &docs_path),
            None => println!("Cannot find the document"),
        }
    }
}

#[test]
fn create_sorted_summary() {
    let mut documents: HashMap<String, Document> = HashMap::new();

    documents.insert(
        "b".to_string(),
        Document {
            title: "b".to_string(),
            file_name: "".to_string(),
            content: "".to_string(),
        },
    );

    documents.insert(
        "a".to_string(),
        Document {
            title: "a".to_string(),
            file_name: "".to_string(),
            content: "".to_string(),
        },
    );

    documents.insert(
        "c".to_string(),
        Document {
            title: "c".to_string(),
            file_name: "".to_string(),
            content: "".to_string(),
        },
    );

    let result = create_summary(&documents);

    assert_eq!(result, "# Summary\n\n* [a](./)\n* [b](./)\n* [c](./)\n");
}
