use crate::query::{
    build::{get_build_method, BuildMethod},
    get_repos,
};
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

pub fn build(
    pkg_path: &Path,
    query: &[String],
    option: &Option<&Vec<String>>,
) -> Result<(), String> {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        Err(format!("no packages satisfy query '{}'", query.join(" ")))
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                if set_current_dir(&repo).is_ok() {
                    match get_build_method(&repo) {
                        Some(BuildMethod::Make) => {
                            println!("{}", rel_path.display());
                            let mut cmd = Command::new("make");
                            if let Some(option) = option {
                                cmd.args(&**option);
                                println!("> make {}", option.join(" "));
                            } else {
                                println!("> make");
                            }
                            match cmd.env("PWD", &repo).status() {
                                Ok(result) => {
                                    if result.success() {
                                        count += 1;
                                    } else {
                                        println!("make failed");
                                    }
                                }
                                Err(msg) => println!("error: {}", msg),
                            }
                        }
                        Some(BuildMethod::Cargo) => {
                            println!("{}", rel_path.display());
                            let mut cmd = Command::new("cargo");
                            cmd.args(&["build", "--release"]);
                            if let Some(option) = option {
                                cmd.args(&["--features", &option.join(",")]);
                                println!("> cargo build --release --features {}", option.join(","));
                            } else {
                                println!("> cargo build --release");
                            }
                            match cmd.status() {
                                Ok(result) => {
                                    if result.success() {
                                        count += 1;
                                    } else {
                                        println!("build failed");
                                    }
                                }
                                Err(msg) => println!("error: {}", msg),
                            }
                        }
                        None => {}
                    }
                } else {
                    println!("failed to access '{}'", repo.display());
                }
            }
        }
        println!("{} packages built", count);
        Ok(())
    }
}
