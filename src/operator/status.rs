use crate::query::{
    get_repos,
    status::{get_status, GitAction},
};
use std::path::Path;

pub fn status(pkg_path: &Path, query: &[String], color: bool) {
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        let mut output = String::new();

        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                if let Some(info) = get_status(&repo) {
                    let mut header = false;

                    let has_commits = info.commits_ahead > 0 || info.commits_behind > 0;

                    if has_commits || info.files.len() > 0 {
                        header = true;
                    } else if let Some(branch) = &info.branch {
                        if &*branch != "master" {
                            header = true;
                        }
                    }

                    if !header {
                        if let Some(upstream) = &info.upstream {
                            if upstream != "origin/master" {
                                header = true;
                            }
                        } else {
                            header = true;
                        }
                    }

                    if header {
                        output.push_str(&rel_path.to_string_lossy());

                        if let Some(branch) = info.branch {
                            if color {
                                if &branch == "master" {
                                    output.push_str(&format!(" \u{1b}[33m@{}\u{1b}[m", branch));
                                } else {
                                    output.push_str(&format!(" \u{1b}[93m@{}\u{1b}[m", branch));
                                }
                            } else {
                                output.push_str(&format!(" @{}", branch));
                            }
                        }

                        if let Some(upstream) = info.upstream {
                            if color && has_commits {
                                output.push_str(&format!(
                                    " \u{1b}[1m\u{1b}[34m[{}:{}]\u{1b}[m",
                                    info.commits_ahead, info.commits_behind
                                ));
                            } else {
                                output.push_str(&format!(
                                    " [{}:{}]",
                                    info.commits_ahead, info.commits_behind
                                ));
                            }

                            output.push_str(&format!(" ({})", upstream));
                        } else {
                            output.push_str(" (no remote)");
                        }

                        output.push('\n');

                        for file in info.files {
                            if color {
                                output.push_str(&format!(
                                    "  {}{}: {}\n",
                                    match file.staged {
                                        true => "+",
                                        false => "-",
                                    },
                                    match file.action {
                                        GitAction::Delete => "\u{1b}[31mD\u{1b}[m",
                                        GitAction::New => "\u{1b}[32mN\u{1b}[m",
                                        GitAction::Modify => "\u{1b}[33mM\u{1b}[m",
                                    },
                                    file.location
                                ));
                            } else {
                                output.push_str(&format!(
                                    "  {}{}: {}\n",
                                    match file.staged {
                                        true => "+",
                                        false => "-",
                                    },
                                    match file.action {
                                        GitAction::Delete => "D",
                                        GitAction::New => "N",
                                        GitAction::Modify => "M",
                                    },
                                    file.location
                                ));
                            }
                        }
                    }
                } else {
                    output.push_str(&format!(
                        "{}\n  error reading git status\n",
                        rel_path.display()
                    ));
                }
            }
        }

        if output.is_empty() {
            println!("all clean");
        } else {
            print!("{}", output);
        }
    }
}
