mod config;
mod generator;
mod parser;

fn main() {
    match config::read_config() {
        Some(config) => {
            let mut paths: Vec<String> = vec![];
            for pattern in &config.files_patterns {
                paths.push(format!("{}/{}", config.project_path, pattern));
            }

            let articles = parser::parse_path(paths, config.clone());

            generator::generate_docs(articles, config)
        }
        None => println!("Cannot find the config file"),
    }
}
