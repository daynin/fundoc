mod cli;
mod config;
mod generator;
mod parser;

use ansi_term::Colour;

fn main() {
    if cli::create_cli().is_present("init") {
        config::create_default_config()
    } else {
        match config::read_config() {
            Some(config) => {
                println!("Start documentation parsing...\n");

                let files_patterns: Vec<String> = vec![
                    vec!["**/*.fdoc.md".to_string()],
                    config.files_patterns.clone(),
                ]
                .concat();
                let mut paths: Vec<String> = vec![];

                for pattern in files_patterns {
                    paths.push(format!("{}/{}", config.project_path, pattern));
                }

                let result = parser::parse_path(paths, config.clone());
                generator::generate_docs(result.articles, config);

                println!(
                    "\n{} {}%",
                    Colour::Green.bold().paint("Documentation coverage:"),
                    result.coverage
                );
                println!("{}", Colour::Green.bold().paint("Done!"));
            }
            None => println!("Cannot find the config file"),
        }
    }
}
