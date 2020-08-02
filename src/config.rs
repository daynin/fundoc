use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

/**
 * @Article Configuration
 *
 * Configuration parameters:
 */
#[derive(Deserialize, Debug)]
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
    /**
     * @Article Configuration
     *
     * - `repository_host` - an http url which will be used for creating a link to a file in a
     * repository. For example, if you want to add links to your files for each section you can pass
     * a value like `https://github.com/user_name/project_name/blob/master`. It will be used for
     * creating an url like this
     * `https://github.com/user_name/project_name/blob/master/path/to/your/file.txt`.
     */
    pub repository_host: Option<String>,
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
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };

    config
}
