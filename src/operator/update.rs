use crate::query::get_repos;
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

pub fn update(pkg_path: &Path, query: &[String]) -> Result<(), String> {
    let mut cloned_ids: Vec<String> = Vec::new();
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        Err(format!("no packages satisfy query '{}'", query.join(" ")))
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                println!("{}", rel_path.display());
                if set_current_dir(&repo).is_ok() {
                    match Command::new("git").arg("pull").status() {
                        Ok(result) => {
                            if result.success() {
                                cloned_ids.push(rel_path.to_string_lossy().to_string());
                            } else {
                                eprintln!("git pull failed");
                            }
                        }
                        Err(msg) => eprintln!("error: {}", msg),
                    }
                } else {
                    eprintln!("failed to access {}", repo.display());
                }
            }
        }
        println!("{} packages updated", &cloned_ids.len());
        Ok(())
    }
}
