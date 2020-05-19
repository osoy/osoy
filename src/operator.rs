use regex::Regex;
use std::path::Path;
use std::process::Command;

mod query;
use query::{get_exes, get_links_to, get_repos, url_from_query};

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

pub fn clone(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    for q in query {
        if let Some(url) = url_from_query(&q) {
            let repo_path = Regex::new("^.*://").unwrap().replace(&url, "");
            if let Some(location) = pkg_path.join(repo_path.as_ref()).to_str() {
                Command::new("git")
                    .args(&["clone", &url, location])
                    .spawn()
                    .expect("git clone failed to start")
                    .wait()
                    .expect("git clone failed");
            } else {
                println!("couldn't map url '{}' to directory path", url);
            }
        } else {
            println!("couldn't build url from query '{}'", q);
        }
    }
}
