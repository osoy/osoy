use regex::{Captures, Regex};
use std::fs::remove_dir;
use std::path::{Path, PathBuf};

pub mod build;
pub mod exes;
pub mod fsmeta;
pub mod status;

fn path_matches_query(path: &Path, query: &[String]) -> bool {
    if query.len() == 0 {
        return true;
    }
    for q in query {
        if path.ends_with(q) {
            return true;
        } else {
            if let Ok(re) = Regex::new(&["/", &q.replace(".", "[^/]"), "$"].join("")) {
                if re.is_match(&["/", &path.to_string_lossy()].join("")) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn get_repos(dir: &Path, prefix: &Path, query: &[String]) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = Vec::new();
    if dir.join(".git").is_dir() {
        if let Ok(rel) = dir.strip_prefix(prefix) {
            if path_matches_query(rel, query) {
                repos.push(dir.to_path_buf());
            }
        }
    } else {
        if let Ok(entries) = dir.read_dir() {
            let mut count = 0;
            for entry in entries {
                count += 1;
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        repos.append(&mut get_repos(&entry_path, prefix, &query));
                    }
                }
            }
            if count == 0 && dir != prefix {
                if remove_dir(dir).is_ok() {
                    eprintln!("info: removed empty directory '{}'", dir.display());
                } else {
                    eprintln!(
                        "warning: failed to remove empty directory {}",
                        dir.display()
                    );
                }
            }
        }
    }
    repos
}

pub fn remove_rec_if_empty(dir: &Path) {
    if let Ok(entries) = dir.read_dir() {
        let mut count = 0;
        for _ in entries {
            count += 1;
        }
        if count == 0 {
            if remove_dir(dir).is_ok() {
                eprintln!("info: removed empty directory {}", dir.display());
                let mut path_buf = dir.to_path_buf();
                path_buf.pop();
                remove_rec_if_empty(&path_buf);
            } else {
                eprintln!(
                    "warning: failed to remove empty directory {}",
                    dir.display()
                );
            }
        }
    }
}

pub fn url_from_query(query: &str) -> Option<String> {
    if query.find("://").is_some() || query.find("@").is_some() {
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
            if domain.len() > 0 && author.len() > 0 && name.len() > 0 {
                return Some(format!("https://{}/{}/{}.git", domain, author, name));
            }
        }
    }
    None
}

pub fn repo_id_from_url(url: &str) -> Option<String> {
    match {
        if url.find("://").is_some() {
            Some(
                Regex::new("^.*://")
                    .unwrap()
                    .replace(&url, "")
                    .to_lowercase(),
            )
        } else if url.find("@").is_some() {
            Some(
                Regex::new("^.*@([^:]+):")
                    .unwrap()
                    .replace(&url, |caps: &Captures| format!("{}/", &caps[1]))
                    .to_lowercase(),
            )
        } else {
            None
        }
    } {
        Some(id) => Some(id.strip_suffix(".git").unwrap_or(&id).to_owned()),
        None => None,
    }
}

pub fn get_first_file(dir: &Path, re: &str) -> Option<PathBuf> {
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file()
                    && Regex::new(re)
                        .unwrap()
                        .is_match(&entry.file_name().to_string_lossy())
                {
                    return Some(entry_path);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_to_url() {
        let url = url_from_query("osoy");
        assert_eq!(url, Some("https://github.com/osoy/osoy.git".to_owned()),);
        let url = url_from_query("rasmusmerzin/colo");
        assert_eq!(
            url,
            Some("https://github.com/rasmusmerzin/colo.git".to_owned())
        );

        let url = url_from_query("https://github.com/osoy/osoy");
        assert_eq!(url, Some("https://github.com/osoy/osoy".to_owned()));
        let url = url_from_query("git@gitlab.com:osoy/osoy");
        assert_eq!(url, Some("git@gitlab.com:osoy/osoy".to_owned()));

        let url = url_from_query("");
        assert_eq!(url, None);
        let url = url_from_query("github.com//osoy");
        assert_eq!(url, None);
    }

    #[test]
    fn url_to_repo_id() {
        let repo_id = repo_id_from_url("https://github.com/osoy/osoy.git");
        assert_eq!(repo_id, Some("github.com/osoy/osoy".to_owned()));
        let repo_id = repo_id_from_url("git@gitlab.com:osoy/osoy.git");
        assert_eq!(repo_id, Some("gitlab.com/osoy/osoy".to_owned()));

        let repo_id = repo_id_from_url("git@gitlab.com:osoy/osoy");
        assert_eq!(repo_id, Some("gitlab.com/osoy/osoy".to_owned()));

        let repo_id = repo_id_from_url("gitlab.com/osoy/osoy");
        assert_eq!(repo_id, None);
        let repo_id = repo_id_from_url("");
        assert_eq!(repo_id, None);
    }
}
