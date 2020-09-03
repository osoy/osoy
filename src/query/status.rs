use regex::Regex;
use std::env::set_current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

#[derive(PartialEq)]
pub enum GitAction {
    Delete,
    Modify,
    New,
    Rename,
}

pub struct GitFile {
    pub location: String,
    pub action: GitAction,
    pub staged: bool,
}

#[derive(Default)]
pub struct GitStatus {
    pub files: Vec<GitFile>,
    pub upstream: Option<String>,
    pub commits_ahead: u32,
    pub commits_behind: u32,
    pub branch: Option<String>,
}

#[derive(PartialEq)]
enum GitStatusSection {
    Description,
    Staged,
    Unstaged,
    Untracked,
}

pub fn get_status(dir: &Path) -> Option<GitStatus> {
    if set_current_dir(&dir).is_ok() {
        if let Ok(output) = Command::new("git").arg("status").output() {
            let mut git_status = GitStatus::default();

            let out = String::from_utf8_lossy(&output.stdout);
            let mut section = GitStatusSection::Description;

            if let Some(caps) = Regex::new(r#"On branch (\S+)"#).unwrap().captures(&out) {
                git_status.branch = Some(caps[1].to_owned());
            }

            if let Some(caps) = Regex::new(
                r#"Your branch is ahead of '([^\n']+)' by ([0-9]+) commit"#,
            )
            .unwrap()
            .captures(&out)
            {
                git_status.commits_ahead = caps[2].parse::<u32>().unwrap_or(0);
                git_status.upstream = Some(caps[1].to_owned());
            } else if let Some(caps) = Regex::new(
                r#"Your branch is behind '([^\n']+)' by ([0-9]+) commit"#,
            )
            .unwrap()
            .captures(&out)
            {
                git_status.commits_behind = caps[2].parse::<u32>().unwrap_or(0);
                git_status.upstream = Some(caps[1].to_owned());
            } else if let Some(caps) = Regex::new(
                "Your branch and '([^\n']+)' have diverged,\nand have ([0-9]+) and ([0-9]+) different commits each, respectively",
            )
            .unwrap().captures(&out) {
                git_status.commits_ahead = caps[2].parse::<u32>().unwrap_or(0);
                git_status.commits_behind = caps[3].parse::<u32>().unwrap_or(0);
                git_status.upstream = Some(caps[1].to_owned());
            } else if let Some(caps) = Regex::new("Your branch is up to date with '([^\n']+)'").unwrap().captures(&out) {
                git_status.upstream = Some(caps[1].to_owned());
            }

            for line in out.lines() {
                if Regex::new("^\t").unwrap().is_match(&line) {
                    match section {
                        GitStatusSection::Staged | GitStatusSection::Unstaged => {
                            if let Some(caps) = Regex::new(r#"^\t+(\S)[^:]+:\s+(\S.*)$"#)
                                .unwrap()
                                .captures(line)
                            {
                                git_status.files.push(GitFile {
                                    location: caps[2].to_owned(),
                                    action: match &caps[1] {
                                        "d" => GitAction::Delete,
                                        "n" => GitAction::New,
                                        "r" => GitAction::Rename,
                                        _ => GitAction::Modify,
                                    },
                                    staged: section == GitStatusSection::Staged,
                                })
                            }
                        }
                        GitStatusSection::Untracked => git_status.files.push(GitFile {
                            location: line.trim_start_matches("\t").to_owned(),
                            action: GitAction::New,
                            staged: false,
                        }),
                        _ => {}
                    }
                } else if line == "Changes to be committed:" {
                    section = GitStatusSection::Staged;
                } else if line == "Changes not staged for commit:" {
                    section = GitStatusSection::Unstaged;
                } else if line == "Untracked files:" {
                    section = GitStatusSection::Untracked;
                }
            }

            return Some(git_status);
        }
    }
    None
}

pub fn get_branch(dir: &Path) -> Option<String> {
    let head = dir.join(".git/HEAD");
    if head.is_file() {
        let mut buffer = String::new();
        if let Ok(f) = File::open(&head) {
            if BufReader::new(f).read_line(&mut buffer).is_ok() {
                return Some(
                    Regex::new("^.*/([^/]+)$")
                        .unwrap()
                        .replace(buffer.trim_end(), "$1")
                        .to_string(),
                );
            }
        }
    }
    None
}
