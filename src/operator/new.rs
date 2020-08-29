use crate::prompt::{prompt_no, Answer};
use crate::query::{repo_id_from_url, url_from_query};
use std::env::set_current_dir;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

pub fn new(pkg_path: &Path, query: &[String], answer: &Answer) {
    if query.len() <= 0 {
        println!("destination required");
    } else {
        for q in query {
            if let Some(url) = url_from_query(&q) {
                let repo_id = repo_id_from_url(&url).unwrap();
                let repo_path = pkg_path.join(&repo_id);
                if !repo_path.exists()
                    || prompt_no(&format!("package '{}' exists. overwrite?", repo_id), answer)
                {
                    if !repo_path.exists() || remove_dir_all(&repo_path).is_ok() {
                        match Command::new("git")
                            .args(&["init", &repo_path.to_string_lossy()])
                            .status()
                        {
                            Ok(result) => {
                                if result.success() {
                                    if set_current_dir(&repo_path).is_ok() {
                                        println!("package created at '{}'", repo_id);
                                        match Command::new("git")
                                            .args(&["remote", "add", "origin", &url])
                                            .status()
                                        {
                                            Ok(result) => {
                                                if result.success() {
                                                    println!("added remote origin '{}'", url);
                                                }
                                            }
                                            Err(msg) => println!("error: {}", msg),
                                        }
                                    }
                                } else {
                                    println!("git init failed");
                                }
                            }
                            Err(msg) => println!("git init failed to start '{}'", msg),
                        }
                    } else {
                        println!("failed to remove package '{}'", repo_id);
                    }
                }
            } else {
                println!("couldn't build url from query '{}'", q);
            }
        }
    }
}