use crate::query::fsmeta::{is_exe, is_symlink};
use std::fs::remove_file;
use std::path::{Path, PathBuf};

fn get_exes(dir: &Path) -> Vec<PathBuf> {
    let mut exes: Vec<PathBuf> = Vec::new();
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

pub fn get_repo_exes(repo: &Path) -> Vec<PathBuf> {
    let mut exes = get_exes(repo);
    for exception in &["target/release", "bin"] {
        exes.extend_from_slice(&get_exes(&repo.join(exception)));
    }
    exes
}

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

pub fn get_links_to(target: &Path, dir: &Path) -> Vec<PathBuf> {
    let mut links: Vec<PathBuf> = Vec::new();
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

fn get_orphan_links(dir: &Path) -> Vec<PathBuf> {
    let mut links: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if is_symlink(&entry_path) {
                    let target = get_node_target(&entry_path);
                    if !is_exe(&target) {
                        links.push(entry_path)
                    }
                }
            }
        }
    }
    links
}

pub fn remove_orphan_links(bin_path: &Path) {
    let mut count = 0;
    for link in get_orphan_links(bin_path) {
        if remove_file(&link).is_ok() {
            count += 1;
        }
    }
    println!("{} links removed", count);
}
