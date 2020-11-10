use crate::config;
use mdbook::config::Config;
use mdbook::MDBook;
use std::path::PathBuf;

pub fn init_book(config: &config::Config) {
    let mut book_cfg = Config::default();
    book_cfg.book.title = Some("My Book".to_string());
    book_cfg.book.authors.push("Michael-F-Bryan".to_string());
    book_cfg.book.src = PathBuf::from("./");
    book_cfg.build.build_dir = PathBuf::from("./book/");

    MDBook::init(config.docs_folder.as_ref().unwrap())
        .create_gitignore(false)
        .with_config(book_cfg)
        .build()
        .expect("Book generation failed");
}

pub fn build_book(config: &config::Config) {
    let md = MDBook::load(config.docs_folder.as_ref().unwrap()).expect("Unable to load the book");
    md.build().expect("Building failed");
}
