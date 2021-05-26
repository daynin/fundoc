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

pub struct ParsingResult {
    pub articles: Vec<Article>,
    pub coverage: f32,
}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.topic == other.topic
            && self.content == other.content
            && self.path == other.path
            && self.start_line == other.start_line
            && self.end_line == other.end_line
    }
}

enum Keywords {
    /**
     * @Article Syntax
     *
     * `@Article <Article name>` is for marking documentation sections to tell in which articale this section should
     * be merged. You can use `markdown` syntax in documentation sections.
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
    /**
    * @Article Syntax
    * `@FileArtcile` allows you to mark a whole file is a source of documentation for a specified
    * article.
    *
    * Example:
    *
    * ```rust
    * /**
    * * @FileArticle How it works
    * */

    * /**
    *  * Documentation for `main`
    *  */
    * fn main() {}
    *
    * /**
    *  * Documentation for `parse_files`
    *  */
    * fn parse_files() {}
    * ```
    * In that case all comments from a file will be parsed in the same way if they were marked with
    * `@Article How it works`
    *
    * If you want to exclude some comment from parsing you need to use `@Ignore` attribute in that
    * section.
    *
    * Example:
    *
    * ```rust
    * /**
    * * @FileArticle How it works
    * */

    * /**
    *  * Documentation for `main`
    *  */
    * fn main() {}
    *
    * /**
    *  * @Ignore
    *  * This comment will be ignored.
    *  */
    * fn parse_files() {}
    * ```
    */
    FileArticle,
    /**
     * @Article Syntax
     * `@Ignore` is for ignoring a marked documentation section.
     */
    Ignore,
}

impl Keywords {
    fn as_str(&self) -> &'static str {
        match *self {
            Keywords::Article => "@Article",
            Keywords::FileArticle => "@FileArticle",
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

fn parse_fdoc_file(file_content: &str, file_path: &str) -> Vec<Article> {
    let file_name = file_path.split('/').last().unwrap();
    let name_chunks: Vec<&str> = file_name.rsplit('.').collect();
    let topic = name_chunks.get(2..).unwrap().join(".");

    vec![Article {
        topic,
        content: String::from(file_content),
        path: String::from(file_path),
        start_line: 1,
        end_line: 1,
    }]
}

fn parse_text(line: &str, comment_symbol: char) -> &str {
    let empty_comment_line = format!("{} ", comment_symbol);
    let trimmed_line = line.trim_start();

    if trimmed_line.starts_with(empty_comment_line.as_str()) {
        trimmed_line.get(2..)
    } else if trimmed_line.starts_with(' ') || trimmed_line.starts_with(comment_symbol) {
        trimmed_line.get(1..)
    } else {
        Some(trimmed_line)
    }
    .unwrap_or("")
}

fn parse_file(file_content: &str, file_path: &str, config: config::Config) -> Vec<Article> {
    if file_path.ends_with(".fdoc.md") {
        return parse_fdoc_file(file_content, file_path);
    }

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
    let mut file_global_topic = String::from("");

    for line in file_content.lines() {
        if line.trim().starts_with(start_comment) {
            is_comment_section = true;
        } else if line.trim().ends_with(end_comment) {
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
            let trimmed_line = trim_article_line(line.to_string(), comment_symbol);

            match trimmed_line {
                _ if trimmed_line.starts_with(Keywords::FileArticle.as_str()) => {
                    file_global_topic = trim_article_line(
                        line.replace(Keywords::FileArticle.as_str(), ""),
                        comment_symbol,
                    );
                }
                _ if !file_global_topic.is_empty() && !is_article_section => {
                    current_article.topic = file_global_topic.clone();
                    current_article.start_line = line_number;
                    is_article_section = true;
                }
                _ if trimmed_line.starts_with(Keywords::Article.as_str()) => {
                    let topic = line.replace(Keywords::Article.as_str(), "");

                    current_article.topic = trim_article_line(topic, comment_symbol);
                    current_article.start_line = line_number;
                    is_article_section = true;
                }
                _ if trimmed_line.starts_with(Keywords::Ignore.as_str()) => {
                    is_article_section = false;
                    is_comment_section = false;
                    current_article = new_article();
                }
                _ if is_article_section => {
                    current_article.content +=
                        format!("{}\n", parse_text(line, comment_symbol)).as_str();
                }
                _ => {}
            };
        }

        line_number += 1;
    }

    articles
}

pub fn parse_path(directory_paths: Vec<String>, config: config::Config) -> ParsingResult {
    let mut result: Vec<Article> = vec![];
    let mut files_with_documentation = 0.0;
    let mut files_counter = 0.0;

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
                    let articles =
                        &mut parse_file(prepared_content.as_str(), file_path, config.clone());

                    files_counter += 1.0;
                    if !articles.is_empty() {
                        files_with_documentation += 1.0;
                    }

                    result.append(articles);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }

    ParsingResult {
        articles: result,
        coverage: files_with_documentation / files_counter * 100.0,
    }
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
        mdbook: None,
        book_name: None,
        book_build_dir: None,
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
        start_line: 3,
        end_line: 4,
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
        start_line: 5,
        end_line: 7,
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
fn parse_articles_with_code_blocks_with_identation() {
    let file_content = "
use std::io::prelude::*;

/**
 * @Article Test article
 * ```rust
 * fn main() {
 *     println!(\"Hello world!\");
 * }
 * ```
 *
 * ```rust
 * fn test() {
 *     println!(\"Hello world!\");
 * }
 * ```
 */
pub fn test () {}
  ";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("```rust\nfn main() {\n    println!(\"Hello world!\");\n}\n```\n\n```rust\nfn test() {\n    println!(\"Hello world!\");\n}\n```"),
        path: "".to_string(),
        start_line: 5,
        end_line: 16,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn parse_documentation_with_identation_before_comments() {
    let file_content = "
     /**
     * @Article Test article
     * #### [no-implicit-coercion](https://eslint.org/docs/rules/no-implicit-coercion)
     * All implicit coercions except `!!` are disallowed:
     * ```js
     * // Fail
     * +foo
     * 1 * foo
     * '' + foo
     * `${foo}`
     * ~foo.indexOf(bar)
     *
     * // Pass
     * !!foo
     * ```
     */
  ";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("#### [no-implicit-coercion](https://eslint.org/docs/rules/no-implicit-coercion)\nAll implicit coercions except `!!` are disallowed:\n```js\n// Fail\n+foo\n1 * foo\n\'\' + foo\n`${foo}`\n~foo.indexOf(bar)\n\n// Pass\n!!foo\n```"),
        path: "".to_string(),
        start_line: 3,
        end_line: 16,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn parse_articles_with_markdown_lists() {
    let file_content = "
use std::io::prelude::*;

/**
 * @Article Test article
 * List:
 * * Item 1
 * * Item 2
 *
 *   Item 2 subtext
 * * Item 3
 */
pub fn test () {}
  ";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("List:\n* Item 1\n* Item 2\n\n  Item 2 subtext\n* Item 3"),
        path: "".to_string(),
        start_line: 5,
        end_line: 11,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn ignore_empty_lines() {
    let file_content = "
use std::io::prelude::*;

/**
@Article Test article

*/
    ";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from(""),
        path: "".to_string(),
        start_line: 5,
        end_line: 6,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn parse_comments_without_comment_prefixes() {
    let file_content = "
/**
@Article Test article
test
*/
";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("test"),
        path: "".to_string(),
        start_line: 3,
        end_line: 4,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn parse_different_types_of_commnet_endings() {
    let file_content = "
/**
 * @Article Test article
 * test
 * */
const a = 1
const b = 2
";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("test"),
        path: "".to_string(),
        start_line: 3,
        end_line: 4,
    }];

    assert_eq!(articles, expected_result);
}

#[test]
fn use_global_article_attribute() {
    let file_content = "
/**
 * @FileArticle Test article
 */

/**
 * test
 */
... some code here

/**
 * test
 */
... some code here
";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![
        Article {
            topic: String::from("Test article"),
            content: String::from("test"),
            path: "".to_string(),
            start_line: 6,
            end_line: 7,
        },
        Article {
            topic: String::from("Test article"),
            content: String::from("test"),
            path: "".to_string(),
            start_line: 11,
            end_line: 12,
        },
    ];

    assert_eq!(articles, expected_result);
}

#[test]
fn ignore_sections_in_case_of_global_article() {
    let file_content = "
/**
 * @FileArticle Test article
 */

/**
 * test
 */
... some code here

/**
 * @Ignore
 * test
 */
... some code here
";

    let articles = parse_file(file_content, "", get_test_config());
    let expected_result = vec![Article {
        topic: String::from("Test article"),
        content: String::from("test"),
        path: "".to_string(),
        start_line: 6,
        end_line: 7,
    }];

    assert_eq!(articles, expected_result);
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

#[test]
fn parse_fdoc_file_check() {
    let result = parse_fdoc_file("test", "/some/long/path/to/file.fdoc.md");
    let expected_result = vec![Article {
        topic: String::from("file"),
        content: String::from("test"),
        path: "/some/long/path/to/file.fdoc.md".to_string(),
        start_line: 1,
        end_line: 1,
    }];

    assert_eq!(result, expected_result);
}
