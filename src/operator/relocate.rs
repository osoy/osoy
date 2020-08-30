use crate::operator::symlink;
use crate::prompt::{prompt_no, Answer};
use crate::query::{get_repos, repo_id_from_url, url_from_query};
use std::env::set_current_dir;
use std::fs::rename;
use std::path::Path;
use std::process::Command;

pub fn relocate(pkg_path: &Path, bin_path: &Path, query: &[String], answer: &Answer) {
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
                    if !move_path.exists()
                        || prompt_no(&format!("package '{}' exists. overwrite?", move_id), answer)
                    {
                        if rename(&repo_path, &move_path).is_ok() {
                            println!(
                                "package moved from '{}' to '{}'",
                                rel_path.display(),
                                move_id
                            );
                            if set_current_dir(&move_path).is_ok() {
                                match Command::new("git")
                                    .args(&["remote", "remove", "origin"])
                                    .status()
                                {
                                    Ok(_) => {}
                                    Err(msg) => println!("error: {}", msg),
                                }
                                match Command::new("git")
                                    .args(&["remote", "add", "origin", &move_url])
                                    .status()
                                {
                                    Ok(result) => {
                                        if result.success() {
                                            println!("added remote origin '{}'", move_url);
                                        }
                                    }
                                    Err(msg) => println!("error: {}", msg),
                                }
                            }
                            symlink(pkg_path, bin_path, &[move_dest.clone()], answer);
                        } else {
                            println!("failed to remove package '{}'", move_id);
                        }
                    }
                }
            } else {
                println!("couldn't build url from move destination '{}'", move_dest);
            }
        }
    }
}
