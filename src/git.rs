use std::process::Command;
use std::fs;
use url::{Url};

use crate::config;
use crate::fs_utils;

#[derive(Debug)]
pub struct Project {
    pub path: String,
    pub config: Option<config::Config>,
}

const TMP_REPOSITORIES: &str = "./.tmp_repositories";

pub fn clone_repositories(config: config::Config) -> Vec<Project> {
    config.repositories.unwrap_or_default().into_iter().map(|url| {
        let path = String::from(Url::parse(&url).unwrap().path());
        let repo_name = &path[path.find('/').unwrap() + 1 .. path.rfind(".git").unwrap()];

        let tmp_dir = format!("{}/{}", TMP_REPOSITORIES, repo_name);

        fs_utils::recreate_dir(&tmp_dir).ok();

        Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(&tmp_dir)
            .output()
            .expect("Failed to execute command");

        Project {
            path: tmp_dir.clone(),
            config: config::read_config(Some(&tmp_dir)),
        }
    }).collect()
}

pub fn remove_tmp_repositories() {
    fs::remove_dir_all(TMP_REPOSITORIES).ok();
}
