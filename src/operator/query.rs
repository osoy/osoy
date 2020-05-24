use regex::Regex;
use std::path::{Path, PathBuf};

mod fsmeta;
pub use fsmeta::create_symlink;
use fsmeta::{is_exe, is_symlink};

fn resolve_relative_link(node: &Path, target: &Path) -> PathBuf {
    if let Some(parent) = node.parent() {
        return parent.join(&target);
    } else {
        return target.to_path_buf();
    }
}

fn get_node_target(node: &Path) -> PathBuf {
    match node.read_link() {
        Ok(target) => {
            let target = resolve_relative_link(node, &target);
            if &target == node {
                return target;
            } else {
                return get_node_target(&target);
            }
        }
        Err(_) => node.to_path_buf(),
    }
}

fn path_matches_query(path: &Path, query: &[String]) -> bool {
    if query.len() == 0 {
        return true;
    }
    for q in query {
        if path.ends_with(q) {
            return true;
        } else {
            if let Ok(re) = Regex::new(&["/", &q.replace(".", "[^/]"), "$"].join("")) {
                if let Some(path_str) = path.to_str() {
                    if re.is_match(&["/", path_str].join("")) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn get_repos(dir: &Path, prefix: &Path, query: &[String]) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = vec![];
    if dir.join(".git").is_dir() {
        if let Ok(rel) = dir.strip_prefix(prefix) {
            if path_matches_query(rel, query) {
                repos.push(dir.to_path_buf());
            }
        }
    } else {
        if let Ok(entries) = dir.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        repos.append(&mut get_repos(&entry_path, prefix, &query));
                    }
                }
            }
        }
    }
    repos
}

pub fn get_exes(dir: &Path) -> Vec<PathBuf> {
    let mut exes: Vec<PathBuf> = vec![];
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if is_exe(&entry_path) {
                        exes.push(entry_path);
                    }
                }
            }
        }
    }
    exes
}

pub fn get_links_to(target: &Path, dir: &Path) -> Vec<PathBuf> {
    let mut links: Vec<PathBuf> = vec![];
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if &get_node_target(&entry_path) == &get_node_target(target) {
                    links.push(entry_path)
                }
            }
        }
    }
    links
}

pub fn get_orphan_links(dir: &Path) -> Vec<PathBuf> {
    let mut links: Vec<PathBuf> = vec![];
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if is_symlink(&entry_path) {
                    if !get_node_target(&entry_path).exists() {
                        links.push(entry_path)
                    }
                }
            }
        }
    }
    links
}

pub fn url_from_query(query: &str) -> Option<String> {
    if query.find("://").is_some() {
        return Some(String::from(query));
    } else {
        let query = query.split("/").collect::<Vec<&str>>();
        let mut iter = query.iter().rev();
        if let Some(name) = iter.next() {
            let author: &str;
            let domain: &str;
            if let Some(pkg_author) = iter.next() {
                author = &pkg_author;
                if let Some(pkg_domain) = iter.next() {
                    domain = &pkg_domain;
                } else {
                    domain = "github.com";
                }
            } else {
                author = &name;
                domain = "github.com";
            }
            return Some(format!("https://{}/{}/{}", domain, author, name));
        }
    }
    None
}