use crate::prompt::{prompt_no, Answer};
use crate::query::{repo_id_from_url, url_from_query};
use std::env::set_current_dir;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

pub fn fork(pkg_path: &Path, query: &[String], answer: &Answer) -> Result<(), String> {
    if query.len() <= 0 {
        Err(format!("query and fork destination required"))
    } else if query.len() <= 1 {
        Err(format!("fork destination required"))
    } else {
        let q = &query[0];
        let fork_dest = &query[1];
        if let Some(url) = url_from_query(&q) {
            if let Some(fork_url) = url_from_query(&fork_dest) {
                let repo_id = repo_id_from_url(&fork_url).unwrap();
                let repo_path = pkg_path.join(&repo_id);
                if !repo_path.exists()
                    || prompt_no(&format!("{} exists. overwrite?", repo_id), answer)
                {
                    if !repo_path.exists() || remove_dir_all(&repo_path).is_ok() {
                        match Command::new("git")
                            .args(&["clone", &url, &repo_path.to_string_lossy()])
                            .status()
                        {
                            Ok(result) => {
                                if result.success() {
                                    if set_current_dir(&repo_path).is_ok() {
                                        match Command::new("git")
                                            .args(&["remote", "rename", "origin", "upstream"])
                                            .status()
                                        {
                                            Ok(_) => {}
                                            Err(msg) => eprintln!("error: {}", msg),
                                        }
                                        match Command::new("git")
                                            .args(&["remote", "add", "origin", &fork_url])
                                            .status()
                                        {
                                            Ok(result) => {
                                                if result.success() {
                                                    println!("added origin {}", fork_url);
                                                }
                                            }
                                            Err(msg) => eprintln!("error: {}", msg),
                                        }
                                    }
                                    Ok(())
                                } else {
                                    Err(format!("git clone failed"))
                                }
                            }
                            Err(msg) => Err(format!("error: {}", msg)),
                        }
                    } else {
                        Err(format!("failed to remove {}", repo_id))
                    }
                } else {
                    Ok(())
                }
            } else {
                Err(format!(
                    "could not build url from fork destination '{}'",
                    fork_dest
                ))
            }
        } else {
            Err(format!("could not build url from query '{}'", q))
        }
    }
}
