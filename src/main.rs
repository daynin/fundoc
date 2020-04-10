mod parser;
mod generator;
mod config;

fn main() {
  match config::read_config() {
    Some(config) => {
      let path = format!("{}/{}", config.project_path, config.files_pattern);
      let articles = parser::parse_path(&path);

      generator::generate_docs(articles, config.docs_folder)
    },
    None => println!("Cannot find the config file"),
  }
}

