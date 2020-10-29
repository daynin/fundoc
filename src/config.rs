use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

/**
 * @Article Configuration
 *
 * Configuration parameters:
 */
#[derive(Deserialize, Serialize, Debug, Clone)]
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
    /**
     * @Article Configuration
     *
     * - `comment_start_string` - a string which marks the start of a comments block. Example: &#47;\*\*
     * - `comment_prefix` - a comment line prefix. Example: \*
     * - `comment_end_string` - a string which marks the end of a comments block. Example: \*&#47;
     */
    pub comment_start_string: Option<String>,
    pub comment_prefix: Option<char>,
    pub comment_end_string: Option<String>,
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

pub fn create_default_config() {
    let config = serde_json::to_string_pretty(&Config {
        docs_folder: Some(String::from("./docs")),
        project_path: String::from("./src"),
        files_patterns: vec![String::from("**/*.rs")],
        comment_start_string: None,
        comment_end_string: None,
        comment_prefix: None,
        repository_host: None,
    })
    .unwrap();

    match File::create("./fundoc.json") {
        Ok(mut file) => match file.write_all(&config.as_bytes()) {
            Ok(_) => println!("Initialization is completed!",),
            Err(_) => println!("Cannot create the config file"),
        },
        Err(e) => println!("{:?}", e),
    }
}
