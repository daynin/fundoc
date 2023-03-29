use mdbook::book::{BookItem, Chapter};
use mdbook::preprocess::CmdPreprocessor;
use regex::Regex;
use serde_json::Error;
use std::{env, fs, io, process};

use crate::config;
use crate::lua_runtime;

pub struct Plugins {
    lua_runtime: lua_runtime::LuaRuntime,
    config: config::Config,
}

/**
 * @Article Plugins
 *
 * You can create a plugin for Fundoc to convert any text block of the following code in documentation:
 *
 * ```
 * {{ #your-plugin-name any text here }}
 *
 * ```
 *
 * This insertion can be multiline as well:
 *
 * ```
 * {{ #your-plugin-name
 *  any text here
 * }}
 * 
 * ```
 *
 * To create a plugin for parsing these blocks, you should add a file called `your-plugin-name.html.lua` into the plugins folder. By default, it's `./plugins`, but it's possible to change it in the config file.
 * Then you should add the following strings into the `book.toml` file:
 *
 * ```toml
 * [preprocessor.your-plugin-name]
 * command = "fundoc -e"
 * ```
 * The `-e` artument runs Fundoc in mdBook extension mode.
 *
 * And in your your-plugin-name.html.lua, you should implement text transformation like this:
 *
 * ```lua
 * function transform(text)
 *   result = 'transformted text' 
 * end
 * ```
 *
 * Fundoc calls the `transform` function and passes the parsed text from the marked text block. You can transform it in any way you want.
 *
 */
impl Plugins {
    pub fn new(lua_runtime: lua_runtime::LuaRuntime, config: config::Config) -> Self {
        Self {
            lua_runtime,
            config,
        }
    }

    pub fn run_as_plugin(&self) -> Result<(), Error> {
        if self.config.plugins_dir.is_none() {
            panic!("There's no a plugin directory");
        }

        let paths = fs::read_dir(self.config.plugins_dir.as_ref().unwrap());
        let args: Vec<String> = env::args().collect();

        /**
         * @Article Plugins
         * > [note] mdBook runs Fundoc twice. The first time fundoc should exit with 0 code. The
         * second time Fundoc transforms the text.
         */
        if args.len() > 3 {
            process::exit(0x0100);
        }

        let (ctx, mut book) = CmdPreprocessor::parse_input(io::stdin()).unwrap();
        let preprocessor = self.parse_private_preprocessor_value(format!("{:?}", ctx));

        for file in paths.unwrap().flatten() {
            let file_path = file.path();
            let (Some(preprocessor_value), Some(path_str)) = (&preprocessor, file_path.to_str()) else {
                serde_json::to_writer(io::stdout(), &book)?;
                break;
            };

            if !path_str.contains(preprocessor_value) {
                serde_json::to_writer(io::stdout(), &book)?;
                break;
            };
            let regex_str = String::from(r"\{\{ #") + preprocessor_value + r"[\w\W]*\}\}";
            let re = Regex::new(&regex_str).unwrap();

            let plugin_src = fs::read_to_string(file_path).unwrap();
            book.sections = book
                .sections
                .iter()
                .map(|section| match section {
                    BookItem::Chapter(chapter) => {
                        let mut content = chapter.content.clone();

                        #[allow(clippy::redundant_clone)]
                        for capture in re.captures_iter(&content.clone()) {
                            let src_text = capture.get(0).map_or("", |c| c.as_str()).to_string();

                            let parsed_fragment = self.parse_chapter(
                                preprocessor_value.to_string(),
                                plugin_src.clone(),
                                src_text.clone(),
                            );
                            content = content.replace(&src_text, &parsed_fragment);
                        }

                        BookItem::Chapter(Chapter {
                            content,
                            ..chapter.clone()
                        })
                    }
                    _ => section.clone(),
                })
                .collect();

            serde_json::to_writer(io::stdout(), &book)?;
        }

        Ok(())
    }

    fn parse_private_preprocessor_value(&self, stringified_ctx: String) -> Option<String> {
        let re = Regex::new(r#""preprocessor": Table\(\{"*(.*?) *":"#).unwrap();

        match re.captures(&stringified_ctx) {
            Some(captures) => captures.get(1).map(|c| String::from(c.as_str())),
            _ => None,
        }
    }

    fn parse_chapter(&self, preprocessor: String, lua_src: String, src_text: String) -> String {
        self.lua_runtime.exec(lua_src);

        let preprocessor_header = String::from("{{ #") + &preprocessor;
        let mut extracted_text = src_text.replace(&preprocessor_header, "");
        extracted_text = extracted_text.replace("}}", "");

        self.lua_runtime.call_transform(extracted_text).unwrap()
    }
}
