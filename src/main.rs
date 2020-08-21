mod config;
mod generator;
mod parser;

use ansi_term::Colour;

fn main() {
    match config::read_config() {
        Some(config) => {
            println!("Start documentation parsing...\n");

            let mut paths: Vec<String> = vec![];
            for pattern in &config.files_patterns {
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
