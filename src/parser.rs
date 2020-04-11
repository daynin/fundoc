use regex::Regex;
use glob::glob;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Article {
  pub topic: String,
  pub content: String,
}

impl PartialEq for Article {
  fn eq(&self, other: &Self) -> bool {
    self.topic == other.topic
      && self.content == other.content
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
    format!("{}{}{}//{}{}|//{}{}", multiline_mode, linebreakers, spaces, spaces, disable_comment, spaces, disable_comment).as_str()
  ).unwrap();
  let enable_regex = Regex::new(
    format!("{}{}{}//{}{}|//{}{}", multiline_mode, linebreakers, spaces, spaces, enable_comment, spaces, enable_comment).as_str()
  ).unwrap();

  let start_idx = match disable_regex.find_iter(&text).next() {
    Some(m) => m.start(),
    None => text.len(),
  };

  let end_idx = match enable_regex.find_iter(&text).last() {
    Some(m) => m.end(),
    None => text.len(),
  };


  let mut result = text.clone();

  if start_idx != end_idx {
    result.replace_range(start_idx..end_idx, "");
  }

  result
}

fn find_comments(file_content: &str) -> Vec<String> {
  let comment_regex = Regex::new(r"(?m)^\s*\*[^\n\r]*").unwrap();
  let comment_begin = Regex::new(r"(?m)^\*\*|\*").unwrap();

  let mut result: Vec<String> = vec![];

  for cap in comment_regex.captures_iter(file_content) {
    let raw_text = comment_begin.replace(&cap[0], "\n");
    let raw_text = raw_text.trim();

    result.push(String::from(raw_text));
  }

  result
}

fn create_article(section: Vec<String>) -> Option<Article> {
  let mut topic: Option<String> = None;
  let mut content = String::new();

  for part in section {
    if part.starts_with(Keywords::Ignore.as_str()) {
      topic = None;
      break;
    } else if part.starts_with(Keywords::Article.as_str()) && topic == None {
      let raw_topic = part.replace(Keywords::Article.as_str(), "");
      let raw_topic = raw_topic.trim();

      topic = Some(String::from(raw_topic));
    } else if content.is_empty() {
      content = part;
    } else {
      content += &format!("\n{}", part);
    }
  }

  if topic.is_some() {
    Some(Article {
      topic: topic.unwrap(),
      content: content,
    })
  } else {
    None
  }
}

pub fn parse_file(file_content: &str) -> Vec<Article> {
  let comments = find_comments(file_content);
  let comments = comments.split(|elem| elem == "/");

  let mut result: Vec<Article> = vec![];

  for section in comments {
    let article = create_article(section.to_vec());
    if article.is_some() {
      result.push(article.unwrap());
    }
  }

  result
}

pub fn parse_path(directory_path: &str) -> Vec<Article> {
  let mut result: Vec<Article> = vec![];

  for entry in glob(directory_path).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        let mut f = File::open(path).expect("File not found");

        let mut content = String::new();
        f.read_to_string(&mut content)
            .expect("something went wrong reading the file");

        let prepared_content = remove_ignored_text(content);
        result.append(&mut parse_file(prepared_content.as_str()));
      },
      Err(e) => {
        println!("{:?}", e);
      },
    }
  }

  result
}

// fundoc-disable
#[test]
fn find_comments_in_file() {
  let file_content = "
/**
 * @Article Test article
 * some text
 */
pub fn test () {}
  ";

  let comments = find_comments(file_content);
  let expected_result = [
    String::from("@Article Test article"),
    String::from("some text"),
    String::from("/"),
  ];

  assert_eq!(*comments, expected_result);
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

  let articles = parse_file(file_content);
  let expected_result = vec![Article {
    topic: String::from("Test article"),
    content: String::from("some text"),
  }];


  assert_eq!(articles, expected_result);
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

  let articles = parse_file(file_content);
  let expected_result = vec![Article {
    topic: String::from("Test article"),
    content: String::from("some multiline\nawesome text"),
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
  let file_content = "fn some_fun() {}\n// fundoc-disable\nsome code here\n// fundoc-enable\ntest";
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
