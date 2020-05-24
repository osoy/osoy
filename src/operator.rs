use regex::Regex;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

mod query;
use query::{get_exes, get_links_to, get_repos, url_from_query};

mod prompt;
use prompt::{prompt_no, prompt_yes};

pub fn list(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    for repo in get_repos(pkg_path, pkg_path, &query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            if let Some(repo_id) = rel_path.to_str() {
                println!("{}", repo_id);
            }
        }
        for exe in get_exes(&repo) {
            if let Some(filename_os) = exe.file_name() {
                if let Some(filename) = filename_os.to_str() {
                    let links = get_links_to(&exe, bin_path);
                    if links.len() == 0 {
                        println!("  {}", filename);
                    } else {
                        let mut link_list = String::new();
                        for link in links {
                            if let Some(filename_os) = link.file_name() {
                                if let Some(filename) = filename_os.to_str() {
                                    if !link_list.is_empty() {
                                        link_list.push_str(", ");
                                    }
                                    link_list.push_str(filename);
                                }
                            }
                        }
                        println!("  {} <- {}", filename, link_list);
                    }
                }
            }
        }
    }
}

pub fn clone(pkg_path: &Path, query: &[String]) {
    for q in query {
        if let Some(url) = url_from_query(&q) {
            let repo_path = pkg_path.join(Regex::new("^.*://").unwrap().replace(&url, "").as_ref());
            if let Some(repo_id) = repo_path.to_str() {
                if !repo_path.exists()
                    || prompt_no(&format!("Package '{}' exists. Overwrite?", repo_id))
                {
                    if !repo_path.exists() || remove_dir_all(&repo_path).is_ok() {
                        Command::new("git")
                            .args(&["clone", &url, repo_id])
                            .spawn()
                            .expect("git clone failed to start")
                            .wait()
                            .expect("git clone failed");
                    } else {
                        println!("failed to remove package '{}'", repo_id);
                    }
                }
            } else {
                println!("couldn't map url '{}' to directory path", url);
            }
        } else {
            println!("couldn't build url from query '{}'", q);
        }
    }
}

pub fn remove(pkg_path: &Path, query: &[String]) {
    let mut repos: Vec<PathBuf> = vec![];
    println!("Removing following packages:");
    for repo in get_repos(pkg_path, pkg_path, &query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            if let Some(repo_id) = rel_path.to_str() {
                println!("{}", repo_id);
                repos.push(repo);
            }
        }
    }
    if prompt_yes("Proceed?") {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                if let Some(repo_id) = rel_path.to_str() {
                    match remove_dir_all(&repo) {
                        Ok(_) => println!("package '{}' removed", repo_id),
                        Err(_) => println!("failed to remove package '{}'", repo_id),
                    };
                }
            }
        }
    }
}
