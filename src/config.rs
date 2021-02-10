use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use dialoguer::console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

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
     * - `docs_folder` - a path to a folder which will contain all generated documents.
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
    /**
     * @Article Configuration
     *
     * `mdbook` - if true generates documentation in format of [mdBook](https://rust-lang.github.io/mdBook/index.html).
     * `book_name` - a name of the result book.
     * `book_build_dir` - a directory that contains the build result.
     */
    pub mdbook: Option<bool>,
    pub book_name: Option<String>,
    pub book_build_dir: Option<String>,
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
    let theme = ColorfulTheme {
        values_style: Style::new().cyan(),
        ..ColorfulTheme::default()
    };

    let gh_username: String = Input::with_theme(&theme)
        .with_prompt("Github username or organization")
        .interact()
        .unwrap();

    let gh_repo: String = Input::with_theme(&theme)
        .with_prompt("Github repository name")
        .interact()
        .unwrap();

    let project_path: String = Input::with_theme(&theme)
        .with_prompt("Project path")
        .default("./src".to_string())
        .interact()
        .unwrap();

    let mdbook = Confirm::with_theme(&theme)
        .with_prompt("Use mdBook format")
        .default(false)
        .interact()
        .unwrap();

    let docs_folder: Option<String> = Input::with_theme(&theme)
        .with_prompt("Docs folder")
        .default(if mdbook { "./docs_src" } else { "./docs" }.to_string())
        .interact()
        .ok();

    let book_name: Option<String> = if mdbook {
        Input::with_theme(&theme)
            .with_prompt("Book name")
            .default(gh_repo.clone())
            .interact()
            .ok()
    } else {
        None
    };

    let book_build_dir: Option<String> = if mdbook {
        Input::with_theme(&theme)
            .with_prompt("Book build directory")
            .default("./docs".to_string())
            .interact()
            .ok()
    } else {
        None
    };

    let repository_host = Some(format!(
        "https://github.com/{}/{}/blob/master/",
        gh_username, gh_repo
    ));

    let config = serde_json::to_string_pretty(&Config {
        docs_folder,
        project_path,
        repository_host,
        book_name,
        book_build_dir,
        mdbook: Some(mdbook),
        files_patterns: vec![String::from("**/*.rs")],
        comment_start_string: None,
        comment_end_string: None,
        comment_prefix: None,
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
