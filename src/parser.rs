use glob::glob;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

use crate::config;

#[derive(Debug)]
pub struct Article {
    pub topic: String,
    pub content: String,
    pub path: String,
    pub start_line: i16,
    pub end_line: i16,
}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.topic == other.topic && self.content == other.content
    }
}

enum Keywords {
    /**
     * @Article Syntax
     *
     * There are only two keywords for writing fundoc docstrings (for now):
     *
     * - `@Article <Article name>` for marking documentation sections to tell in which articale this section should
     * be merged. You can use `markdown` syntax in documentation sections.
     * - `@Ignore` for ignoring a marked documentation section.
     *
     * Example:
     *
     * ```rust
     * /**
     *  * @Article How it works
     *  *
     *  * # Title of the article
     *  *
     *  * Some text
     *  */
     * fn main() {}
     * ```
     */
    Article,
    Ignore,
}

impl Keywords {
    fn as_str(&self) -> &'static str {
        match *self {
            Keywords::Article => "@Article",
            Keywords::Ignore => "@Ignore",
        }
    }
}

/**
 * @Article Configuration
 *
 * You can diable parsing for a part of your file or a whole file by adding this comment: `fundoc-disable`.
 * If you wan't to turn fundoc on few lines below just add this comment: `fundoc-enable`.
 *
 * In case when you don't write the enable-comment all text from disable comment until the end of
 * the file will be ignored
 */
fn remove_ignored_text(text: String) -> String {
    let multiline_mode = r"(?m)";
    let linebreakers = r"[\n\r]+";
    let spaces = r"\s*";
    let disable_comment = "fundoc-disable";
    let enable_comment = "fundoc-enable";

    let disable_regex = Regex::new(
        format!(
            "{}{}{}//{}{}|//{}{}",
            multiline_mode, linebreakers, spaces, spaces, disable_comment, spaces, disable_comment
        )
        .as_str(),
    )
    .unwrap();
    let enable_regex = Regex::new(
        format!(
            "{}{}{}//{}{}|//{}{}",
            multiline_mode, linebreakers, spaces, spaces, enable_comment, spaces, enable_comment
        )
        .as_str(),
    )
    .unwrap();

    let start_idx = match disable_regex.find_iter(&text).next() {
        Some(m) => m.start(),
        None => text.len(),
    };

    let end_idx = match enable_regex.find_iter(&text).last() {
        Some(m) => m.end(),
        None => text.len(),
    };

    let mut result = text;

    if start_idx != end_idx {
        result.replace_range(start_idx..end_idx, "");
    }

    result
}

fn trim_article_line(line: String, comment_symbol: char) -> String {
    line.trim_start()
        .trim_start_matches(comment_symbol)
        .trim_start()
        .to_string()
}

fn new_article() -> Article {
    Article {
        topic: String::from(""),
        content: String::from(""),
        path: String::from(""),
        start_line: 1,
        end_line: 1,
    }
}

fn parse_file(file_content: &str, file_path: &str, config: config::Config) -> Vec<Article> {
    let start_comment = &config
        .comment_start_string
        .unwrap_or_else(|| "/**".to_string());
    let comment_symbol = config.comment_prefix.unwrap_or('*');
    let end_comment = &config
        .comment_end_string
        .unwrap_or_else(|| "*/".to_string());

    let mut line_number = 1;
    let mut articles: Vec<Article> = vec![];
    let mut current_article: Article = new_article();
    let mut is_comment_section = false;
    let mut is_article_section = false;

    for line in file_content.lines() {
        if line.trim().starts_with(start_comment) {
            is_comment_section = true;
        } else if line.trim().starts_with(end_comment) {
            is_comment_section = false;
            if is_article_section {
                is_article_section = false;

                current_article.content = current_article.content.trim().to_string();
                current_article.path = file_path.to_string();
                current_article.end_line = line_number - 1;
                articles.push(current_article);

                current_article = new_article();
            }
        }

        if is_comment_section {
            if trim_article_line(line.to_string(), comment_symbol)
                .starts_with(Keywords::Article.as_str())
            {
                let topic = line.replace(Keywords::Article.as_str(), "");

                current_article.topic = trim_article_line(topic, comment_symbol);
                current_article.start_line = line_number;
                is_article_section = true;
            } else if trim_article_line(line.to_string(), comment_symbol)
                .starts_with(Keywords::Ignore.as_str())
            {
                is_article_section = false;
                is_comment_section = false;
                current_article = new_article();
            } else if is_article_section {
                let trimmed_content = trim_article_line(line.to_string(), comment_symbol);

                current_article.content += format!("{}\n", trimmed_content).as_str();
            }
        }

        line_number += 1;
    }

    articles
}

pub fn parse_path(directory_paths: Vec<String>, config: config::Config) -> Vec<Article> {
    let mut result: Vec<Article> = vec![];

    for path in directory_paths {
        for entry in glob(&path).expect("Failed to read glob pattern") {
            match entry {
                Ok(entry_path) => {
                    let mut f = File::open(&entry_path).expect("File not found");

                    let mut content = String::new();
                    f.read_to_string(&mut content)
                        .expect("something went wrong reading the file");

                    let prepared_content = remove_ignored_text(content);
                    let file_path = entry_path.to_str().unwrap();

                    result.append(&mut parse_file(
                        prepared_content.as_str(),
                        file_path,
                        config.clone(),
                    ));
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }

    result
}

// fundoc-disable
#[cfg(test)]
fn get_test_config() -> config::Config {
    config::Config {
        project_path: "test".to_string(),
        files_patterns: vec!["test".to_string()],
        docs_folder: None,
        repository_host: None,
        comment_start_string: None,
        comment_prefix: None,
        comment_end_string: None,
    }
}

#[test]
fn parse_articles_from_file_content() {
    let file_content = "
/**
 * @Article Test article
 * some text
 */
pub fn test () {}
  ";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("some text"),
        path: "".to_string(),
        start_line: 1,
        end_line: 1,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn ignore_comments_with_ignore_mark() {
    let file_content = "
/**
 * @Article Test article
 * @Ignore
 * some text
 *
 * next line
 */
pub fn test () {}
  ";

    let articles = parse_file(file_content, "", get_test_config());

    assert_eq!(articles, vec![]);
}

#[test]
fn parse_articles_with_multiline_content_from_file_content() {
    let file_content = "
use std::io::prelude::*;

/**
 * @Article Test article
 * some multiline
 * awesome text
 */
pub fn test () {}
  ";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("some multiline\nawesome text"),
        path: "".to_string(),
        start_line: 1,
        end_line: 1,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn remove_ignored_text_from_file_content() {
    let file_content = "fn some_fun() {}\n// fundoc-disable\nsome code here";
    let expected_result = "fn some_fun() {}";

    let result = remove_ignored_text(file_content.to_string());

    assert_eq!(result, expected_result);
}

#[test]
fn turn_off_and_on_fundoc() {
    let file_content =
        "fn some_fun() {}\n// fundoc-disable\nsome code here\n// fundoc-enable\ntest";
    let expected_result = "fn some_fun() {}\ntest";

    let result = remove_ignored_text(file_content.to_string());

    assert_eq!(result, expected_result);
}

#[test]
fn turn_off_fundoc_for_whole_file() {
    let file_content = "// fundoc-disable\nfn some_fun() {}\nsome code here\ntest";
    let expected_result = "";

    let result = remove_ignored_text(file_content.to_string());

    assert_eq!(result, expected_result);
}
