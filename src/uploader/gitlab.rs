use std::{env, process::Command};

use crate::config::app_config::AppConfig;

pub fn commit(msg: &str) {
    let current_dir = env::current_dir().expect("can't find current dir");
    // let current_path = current_dir.as_path();

    let app_config = AppConfig::new();

    env::set_current_dir(app_config.class_repo_path).expect("cant set current dir");

    let mut git_add = Command::new("git")
      .args(["add", "."])
      .spawn()
      .expect("git add failed");
    
    git_add.wait().expect("cannot terminate git add process");

    let mut git_commit = Command::new("git").args(["commit", "-m", msg]).spawn().expect("git commit failed");

    git_commit.wait().expect("cannot terminate git commit process");

    env::set_current_dir(current_dir).expect("cant set current dir");

}

pub fn push() {
    let current_dir = env::current_dir().expect("can't find current dir");
    // let current_path = current_dir.as_path();

    let app_config = AppConfig::new();

    env::set_current_dir(app_config.class_repo_path).expect("cant set current dir");

    let mut git_push = Command::new("git")
      .args([
        "push",
        "origin",
        "main"
      ]).spawn().expect("git push failed");

    git_push.wait().expect("failed to terminate git push");

    env::set_current_dir(current_dir).expect("cant set current dir");
}
