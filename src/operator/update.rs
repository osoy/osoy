use crate::operator::{build, symlink};
use crate::prompt::{prompt_yes, Answer};
use crate::query::{build::get_build_method, get_repos};
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

pub fn update(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    let mut cloned_ids: Vec<String> = Vec::new();
    let mut have_makefiles = false;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                println!("{}", rel_path.display());
                if set_current_dir(&repo).is_ok() {
                    match Command::new("git").arg("pull").status() {
                        Ok(result) => {
                            if result.success() {
                                cloned_ids.push(rel_path.to_string_lossy().to_string());
                                if !have_makefiles {
                                    have_makefiles = get_build_method(&repo).is_some()
                                }
                            } else {
                                println!("git pull failed");
                            }
                        }
                        Err(msg) => println!("git pull failed to start '{}'", msg),
                    }
                } else {
                    println!("failed to access '{}'", repo.display());
                }
            }
        }
        println!("{} packages updated", &cloned_ids.len());
        if have_makefiles && prompt_yes("build updated packages?", answer) {
            build(pkg_path, bin_path, &cloned_ids, answer, option);
        } else {
            symlink(pkg_path, bin_path, &cloned_ids, answer);
        }
    }
}
