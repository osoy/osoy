use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

fn get_node_target(node: &Path) -> PathBuf {
    match node.read_link() {
        Ok(target) => get_node_target(target.as_path()),
        Err(_) => node.to_path_buf(),
    }
}

pub fn get_repos(dir: &Path) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = vec![];
    if dir.join(".git").is_dir() {
        repos.push(dir.to_path_buf());
    } else {
        if let Ok(entries) = dir.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        repos.append(&mut get_repos(entry_path.as_path()));
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
                    if let Ok(attr) = entry_path.metadata() {
                        if attr.mode() & 0o100 > 0 {
                            exes.push(entry_path);
                        }
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
                if get_node_target(entry_path.as_path()).as_path() == target {
                    links.push(entry_path)
                }
            }
        }
    }
    links
}
