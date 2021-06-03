use ansi_term::Colour;
use std::env;
use std::fs;
use std::process::Command;
use url::{ParseError, Url};

use crate::config;
use crate::fs_utils;

#[derive(Debug)]
pub struct Project {
    pub path: String,
    pub config: Option<config::Config>,
}

const TMP_REPOSITORIES: &str = "./.tmp_repositories";

fn get_repo_url(url: &str) -> Result<Url, ParseError> {
    Url::parse(url).map(|mut parsed_url| match env::var("GH_TOKEN") {
        Ok(gh_token) => {
            parsed_url.set_username("fundoc").ok();
            parsed_url.set_password(Some(&gh_token)).ok();

            parsed_url
        }
        Err(_) => parsed_url,
    })
}

pub fn clone_repositories(config: config::Config) -> Vec<Project> {
    config
        .repositories
        .unwrap_or_default()
        .into_iter()
        .map(|url| {
            println!("\n{} {}", Colour::Green.bold().paint("Clone"), url);

            let path = String::from(Url::parse(&url).unwrap().path());
            let repo_name = &path[path.find('/').unwrap() + 1..path.rfind(".git").unwrap()];

            let tmp_dir = format!("{}/{}", TMP_REPOSITORIES, repo_name);

            fs_utils::recreate_dir(&tmp_dir).ok();

            Command::new("git")
                .arg("clone")
                .arg(get_repo_url(&url).unwrap().as_str())
                .arg(&tmp_dir)
                .output()
                .expect("Failed to clone the repo.");

            Project {
                path: tmp_dir.clone(),
                config: config::read_config(Some(&tmp_dir)),
            }
        })
        .collect()
}

pub fn remove_tmp_repositories() {
    fs::remove_dir_all(TMP_REPOSITORIES).ok();
}
