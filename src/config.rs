use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize};

/**
 * @Article Configuration
 *
 * Configuration parameters:
 */
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
  /**
   * @Article Configuration
   *
   * - `docs_folder` - a path to a folder which will contain all generated documents. It's an
   * optional parameter so if you won't set it up all documents will be placed in `docs` folder in
   * the working directory.
   *
   * > **NOTE** be careful, all files in the `docs_folder` will be replaced by documentation files.
   */
  pub docs_folder: Option<String>,
  /**
   * @Article Configuration
   *
   * - `project_path` - an entry point for the parser
   */
  pub project_path: String,
  /**
   * @Article Configuration
   *
   * - `files_patterns` - unix style pathname patterns for matching files which will be parsed.
   */
  pub files_patterns: Vec<String>,
}

/**
 * @Article Configuration
 *
 * Fundoc will read all the configuration parameters from the `fundoc.json` config file
 * which should be placed into the working directory of the programm's proccess (generally, it's a root of a
 * poject)
 */
const DEFAULT_CONFIG_PATH: &str = "./fundoc.json";

pub fn read_config() -> Option<Config> {
  let mut config: Option<Config> = None;

  match File::open(DEFAULT_CONFIG_PATH) {
    Ok(mut file) => {
      let mut content = String::new();
      if file.read_to_string(&mut content).is_err() {
        println!("Cannot read config file");
      };

      config = serde_json::from_str(content.as_str()).unwrap();
    },
    Err(e) => {
      println!("{:?}", e);
    }
  };

  config
}
