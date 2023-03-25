mod book;
mod cli;
mod config;
mod fs_utils;
mod generator;
mod git;
mod lua_runtime;
mod parser;
mod plugins;

use ansi_term::Colour;
use plugins::Plugins;
use std::fs;

fn parse_articles(config: config::Config, root: &str) -> Vec<parser::Article> {
    let mut parser = parser::Parser::new(config.clone());
    println!("Start documentation parsing...\n");

    let files_patterns: Vec<String> = vec![
        vec![format!("{}/**/*.fdoc.md", root)],
        config.files_patterns.clone(),
    ]
    .concat();
    let mut paths: Vec<String> = vec![];

    for pattern in files_patterns {
        paths.push(format!("{}/{}/{}", root, config.project_path, pattern));
    }

    let result = parser.parse_path(paths);

    println!(
        "\n{} {}%",
        Colour::Green.bold().paint("Documentation coverage:"),
        result.coverage
    );
    println!("{}", Colour::Green.bold().paint("Done!"));

    result.articles
}

fn generate_book(config: config::Config) {
    /* book::init_book(config); */
    book::build_book();
}

fn main() {
    let args = cli::create_cli();

    if let Some(true) = args.get_one::<bool>("init") {
        config::create_default_config()
    } else if let Some(true) = args.get_one::<bool>("extension") {
        let plugins = plugins::Plugins::new(lua_runtime::LuaRuntime::new());
        plugins.run_as_plugin();
    } else {
        match config::read_config(None) {
            Some(config) => {
                let mut articles: Vec<parser::Article> = vec![];
                fs_utils::recreate_dir(&config.clone().docs_folder.unwrap())
                    .expect("Cannot create the documentation folder");
                articles.append(&mut parse_articles(config.clone(), "."));

                for project in git::clone_repositories(config.clone()) {
                    if project.config.is_some() {
                        articles.append(&mut parse_articles(project.config.unwrap(), &project.path))
                    }
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
