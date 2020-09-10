use crate::query::{
    get_repos,
    status::{get_status, GitAction},
};
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn status(pkg_path: &Path, query: &[String], color: bool, quiet: bool) {
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        let mut clean = true;
        let working_dir = current_dir().unwrap_or(PathBuf::new());

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
                        clean = false;
                        let mut output = String::new();
                        if color && working_dir == repo {
                            output.push_str(&format!("\u{1b}[1m{}\u{1b}[m", rel_path.display()));
                        } else {
                            output.push_str(&rel_path.to_string_lossy());
                        }

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
                                    " \u{1b}[1m\u{1b}[94m[{}:{}]\u{1b}[m",
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

                        if quiet && info.files.len() > 0 {
                            let mut deleted_count = 0;
                            let mut created_count = 0;
                            let mut modified_count = 0;
                            let mut renamed_count = 0;

                            for file in &info.files {
                                match file.action {
                                    GitAction::Delete => deleted_count += 1,
                                    GitAction::New => created_count += 1,
                                    GitAction::Modify => modified_count += 1,
                                    GitAction::Rename => renamed_count += 1,
                                };
                            }

                            if color {
                                if deleted_count > 0 {
                                    output.push_str(&format!(
                                        " \u{1b}[31mD{}\u{1b}[m",
                                        deleted_count
                                    ));
                                }
                                if created_count > 0 {
                                    output.push_str(&format!(
                                        " \u{1b}[32mN{}\u{1b}[m",
                                        created_count
                                    ));
                                }
                                if modified_count > 0 {
                                    output.push_str(&format!(
                                        " \u{1b}[33mM{}\u{1b}[m",
                                        modified_count
                                    ));
                                }
                                if renamed_count > 0 {
                                    output.push_str(&format!(
                                        " \u{1b}[34mR{}\u{1b}[m",
                                        renamed_count
                                    ));
                                }
                            } else {
                                if deleted_count > 0 {
                                    output.push_str(&format!(" D{}", deleted_count));
                                }
                                if created_count > 0 {
                                    output.push_str(&format!(" N{}", created_count));
                                }
                                if modified_count > 0 {
                                    output.push_str(&format!(" M{}", modified_count));
                                }
                                if renamed_count > 0 {
                                    output.push_str(&format!(" R{}", renamed_count));
                                }
                            }
                        }

                        output.push('\n');

                        if !quiet {
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
                                            GitAction::Rename => "\u{1b}[34mR\u{1b}[m",
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
                                            GitAction::Rename => "R",
                                        },
                                        file.location
                                    ));
                                }
                            }
                        }

                        print!("{}", output);
                    }
                } else {
                    clean = false;
                    println!("{}\n  error reading git status", rel_path.display());
                }
            }
        }

        if clean {
            println!("OK");
        }
    }
}
