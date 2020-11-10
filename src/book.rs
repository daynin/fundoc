use crate::config;
use mdbook::config::Config;
use mdbook::MDBook;
use std::path::PathBuf;

pub fn init_book(config: config::Config) {
    let mut book_cfg = Config::default();
    book_cfg.book.title = config.book_name;
    book_cfg.book.src = PathBuf::from(config.book_src.unwrap());
    book_cfg.build.build_dir = PathBuf::from(config.book_build_dir.unwrap());

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
