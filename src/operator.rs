use std::path::Path;

mod query;
use query::{get_exes, get_links_to, get_repos};

pub fn list(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    for repo in get_repos(pkg_path, pkg_path, &query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            if let Some(repo_id) = rel_path.to_str() {
                println!("{}", repo_id);
            }
        }
        for exe in get_exes(repo.as_path()) {
            if let Some(filename_os) = exe.file_name() {
                if let Some(filename) = filename_os.to_str() {
                    let links = get_links_to(exe.as_path(), bin_path);
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
