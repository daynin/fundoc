use mdbook::book::BookItem;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use regex::Regex;
use std::{env, fs, io, process};

use crate::lua_runtime;

pub struct Plugins {
    lua_runtime: lua_runtime::LuaRuntime,
}

fn between<'value>(value: &'value str, a: &str, b: &str) -> &'value str {
    // Find the two strings.
    if let Some(pos_a) = value.find(a) {
        if let Some(pos_b) = value.rfind(b) {
            // Return the part in between the 2 strings.
            let adjusted_pos_a = &pos_a + a.len();
            if adjusted_pos_a < pos_b {
                return &value[adjusted_pos_a..pos_b];
            }
        }
    }
    return "";
}

impl Plugins {
    pub fn new(lua_runtime: lua_runtime::LuaRuntime) -> Self {
        Self { lua_runtime }
    }

    pub fn run_as_plugin(&self) {
        let paths = fs::read_dir("./plugins/preprocessors/");
        let args: Vec<String> = env::args().collect();

        if args.len() > 3 {
            process::exit(0x0100);
        }

        let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).unwrap();

        let re = Regex::new(r"\{\{ #mermaid[\w\W]*\}\}").unwrap();

        match paths {
            Ok(paths) => {
                for file in paths {
                    match file {
                        Ok(file) => {
                            for section in book.sections.iter() {
                                match section {
                                    BookItem::Chapter(chapter) => {
                                        for capture in re.captures(&chapter.content) {
                                            let plugin_src =
                                                fs::read_to_string(&file.path()).unwrap();
                                            self.lua_runtime.exec(plugin_src, |ctx| {
                                                let globals = ctx.globals();
                                                let text = capture.get(0).map_or("", |c| c.as_str());
                                                globals.set(
                                                    "parsed_chunk",
                                                    text,
                                                );
                                            });
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
