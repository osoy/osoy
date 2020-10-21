use crate::prompt::{prompt_no, Answer};
use crate::query::{get_repos, repo_id_from_url, url_from_query};
use std::env::set_current_dir;
use std::fs::{create_dir_all, remove_dir_all, rename};
use std::path::Path;
use std::process::Command;

pub fn relocate(pkg_path: &Path, query: &[String], answer: &Answer) {
    if query.len() <= 0 {
        println!("query and move destination required");
    } else if query.len() <= 1 {
        println!("move destination required");
    } else {
        let (move_dest, query) = query.split_last().unwrap();
        let repos = get_repos(pkg_path, pkg_path, query);
        if repos.len() <= 0 {
            println!("no packages satisfy query '{}'", query.join(" "));
        } else if repos.len() > 1 {
            println!("query is ambigious '{}'", query.join(" "));
        } else {
            let repo_path = &repos[0];
            if let Some(move_url) = url_from_query(&move_dest) {
                if let Ok(rel_path) = repo_path.strip_prefix(pkg_path) {
                    let move_id = repo_id_from_url(&move_url).unwrap();
                    let move_path = pkg_path.join(&move_id);
                    if &move_path == repo_path
                        || !move_path.exists()
                        || prompt_no(&format!("package '{}' exists. overwrite?", move_id), answer)
                    {
                        if &move_path != repo_path {
                            if move_path.exists() {
                                match remove_dir_all(&move_path) {
                                    Ok(_) => println!("removed package '{}'", move_id),
                                    Err(e) => {
                                        println!("failed to remove package '{}': {}", move_id, e)
                                    }
                                }
                            }
                            match create_dir_all(&move_path) {
                                Ok(_) => println!("created empty directory '{}'", move_id),
                                Err(e) => {
                                    println!("failed to create directory '{}': {}", move_id, e)
                                }
                            }
                        }
                        match rename(&repo_path, &move_path) {
                            Ok(_) => {
                                println!(
                                    "package moved from '{}' to '{}'",
                                    rel_path.display(),
                                    move_id
                                );
                                if set_current_dir(&move_path).is_ok() {
                                    match Command::new("git")
                                        .args(&["remote", "set-url", "origin", &move_url])
                                        .status()
                                    {
                                        Ok(result) => {
                                            if result.success() {
                                                println!("renamed remote origin '{}'", move_url);
                                            }
                                        }
                                        Err(msg) => println!("error: {}", msg),
                                    }
                                }
                            }
                            Err(e) => {
                                println!("failed to rename package '{}': {}", rel_path.display(), e)
                            }
                        }
                    }
                }
            } else {
                println!("couldn't build url from move destination '{}'", move_dest);
            }
        }
    }
}
