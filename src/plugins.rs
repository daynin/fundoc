use mdbook::book::{BookItem, Chapter};
use mdbook::preprocess::CmdPreprocessor;
use regex::Regex;
use std::{env, fs, io, process};

use crate::config;
use crate::lua_runtime;

pub struct Plugins {
    lua_runtime: lua_runtime::LuaRuntime,
    config: config::Config,
}

impl Plugins {
    pub fn new(lua_runtime: lua_runtime::LuaRuntime, config: config::Config) -> Self {
        Self { lua_runtime, config }
    }

    pub fn run_as_plugin(&self) {
        if self.config.plugins_dir.is_none() {
            ()
        }

        let paths = fs::read_dir(self.config.plugins_dir.as_ref().unwrap());
        let args: Vec<String> = env::args().collect();

        if args.len() > 3 {
            process::exit(0x0100);
        }

        let (ctx, mut book) = CmdPreprocessor::parse_input(io::stdin()).unwrap();
        eprintln!("{:?}", ctx);

        let re = Regex::new(r"\{\{ #mermaid[\w\W]*\}\}").unwrap();

        for file in paths.unwrap() {
            match file {
                Ok(file) => {
                    let plugin_src = fs::read_to_string(&file.path()).unwrap();
                    book.sections = book.sections.iter().map(|section| {
                        match section {
                            BookItem::Chapter(chapter) => {
                                let mut content = chapter.content.clone();

                                for capture in re.captures(&content.clone()) {
                                    let src_text =
                                        capture.get(0).map_or("", |c| c.as_str()).to_string();

                                    let parsed_fragment = self.parse_chapter(plugin_src.clone(), src_text.clone());
                                    content = content.replace(&src_text, &parsed_fragment);

                                }

                                BookItem::Chapter(Chapter {
                                    content,
                                    ..chapter.clone()
                                })
                            }
                            _ => section.clone()
                        }
                    }).collect();

                    serde_json::to_writer(io::stdout(), &book);
                }
                _ => {}
            }
        }
    }

    fn parse_chapter(&self, lua_src: String, src_text: String) -> String {
        self.lua_runtime.exec(lua_src);

        let mut extracted_text = src_text.replace("{{ #mermaid", "");
        extracted_text = extracted_text.replace("}}", "");

        let result = self.lua_runtime.call_transform(extracted_text).unwrap();

        result
    }
}
