mod book;
mod cli;
mod config;
mod fs_utils;
mod generator;
mod parser;
mod git;
use std::fs;

use ansi_term::Colour;

fn parse_articles(config: config::Config, root: &str) -> Vec<parser::Article> {
    println!("Start documentation parsing...\n");

    let files_patterns: Vec<String> = vec![
        vec![format!("{}/**/*.fdoc.md", root).to_string()],
        config.files_patterns.clone(),
    ]
    .concat();
    let mut paths: Vec<String> = vec![];

    for pattern in files_patterns {
        paths.push(format!("{}/{}/{}", root, config.project_path, pattern));
    }

    let result = parser::parse_path(paths, config.clone());

    println!(
        "\n{} {}%",
        Colour::Green.bold().paint("Documentation coverage:"),
        result.coverage
    );
    println!("{}", Colour::Green.bold().paint("Done!"));

    result.articles
}

fn generate_book(config: config::Config) {
    book::init_book(config);
    book::build_book();
}

fn main() {
    if cli::create_cli().is_present("init") {
        config::create_default_config()
    } else {
        match config::read_config(None) {
            Some(config) => {
                let mut articles: Vec<parser::Article> = vec![];
                fs_utils::recreate_dir(&config.clone().docs_folder.unwrap()).expect("Cannot create the documentation folder");
                articles.append(&mut parse_articles(config.clone(), "."));

                for project in git::clone_repositories(config.clone()) {
                    match project.config {
                        Some(project_config) => articles.append(&mut parse_articles(project_config, &project.path)),
                        None => {},
                    };
                }

                generator::generate_docs(articles, config.clone());

                if config.mdbook.unwrap() {
                    generate_book(config.clone());

                    fs::remove_dir_all(config.docs_folder.unwrap()).ok();
                }

                git::remove_tmp_repositories();
            }
            None => println!("Cannot find the config file"),
        }
    }
}
