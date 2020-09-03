use crate::operator::{build, symlink};
use crate::prompt::{prompt_no, prompt_yes, Answer};
use crate::query::{build::get_build_method, repo_id_from_url, url_from_query};
use std::env::set_current_dir;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

pub fn fork(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    if query.len() <= 0 {
        println!("query and fork destination required");
    } else if query.len() <= 1 {
        println!("fork destination required");
    } else {
        let q = &query[0];
        let fork_dest = &query[1];
        if let Some(url) = url_from_query(&q) {
            if let Some(fork_url) = url_from_query(&fork_dest) {
                let repo_id = repo_id_from_url(&url).unwrap();
                let repo_path = pkg_path.join(&repo_id);
                if !repo_path.exists()
                    || prompt_no(&format!("package '{}' exists. overwrite?", repo_id), answer)
                {
                    if !repo_path.exists() || remove_dir_all(&repo_path).is_ok() {
                        match Command::new("git")
                            .args(&["clone", &url, &repo_path.to_string_lossy()])
                            .status()
                        {
                            Ok(result) => {
                                if result.success() {
                                    println!("package cloned from '{}'", url);
                                    if set_current_dir(&repo_path).is_ok() {
                                        match Command::new("git")
                                            .args(&["remote", "rename", "origin", "upstream"])
                                            .status()
                                        {
                                            Ok(_) => {}
                                            Err(msg) => println!("error: {}", msg),
                                        }
                                        match Command::new("git")
                                            .args(&["remote", "add", "origin", &fork_url])
                                            .status()
                                        {
                                            Ok(result) => {
                                                if result.success() {
                                                    println!("added remote origin '{}'", fork_url);
                                                }
                                            }
                                            Err(msg) => println!("error: {}", msg),
                                        }
                                    }
                                    if get_build_method(&repo_path).is_some()
                                        && prompt_yes("build forked package?", answer)
                                    {
                                        build(
                                            pkg_path,
                                            bin_path,
                                            &[repo_id.clone()],
                                            answer,
                                            option,
                                        );
                                    } else {
                                        symlink(pkg_path, bin_path, &[repo_id.clone()], answer);
                                    }
                                } else {
                                    println!("git clone failed");
                                }
                            }
                            Err(msg) => println!("git clone failed to start '{}'", msg),
                        }
                    } else {
                        println!("failed to remove package '{}'", repo_id);
                    }
                }
            } else {
                println!("couldn't build url from fork destination '{}'", fork_dest);
            }
        } else {
            println!("couldn't build url from query '{}'", q);
        }
    }
}
