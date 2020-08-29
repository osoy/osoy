use crate::operator::symlink;
use crate::prompt::Answer;
use crate::query::{get_repos, has_makefile};
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

pub fn make(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                if set_current_dir(&repo).is_ok() {
                    if has_makefile(&repo) {
                        println!("{}", rel_path.display());
                        let mut cmd = Command::new("make");
                        if let Some(option) = option {
                            cmd.arg(option);
                            println!("> make {}", option);
                        } else {
                            println!("> make");
                        }
                        match cmd.status() {
                            Ok(result) => {
                                if result.success() {
                                    count += 1;
                                } else {
                                    println!("make failed");
                                }
                            }
                            Err(msg) => println!("make failed '{}'", msg),
                        }
                    }
                } else {
                    println!("failed to access '{}'", repo.display());
                }
            }
        }
        println!("{} packages built", count);
        symlink(pkg_path, bin_path, query, answer);
    }
}
