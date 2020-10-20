use crate::query::{
    exes::{get_links_to, get_repo_exes},
    get_repos,
    status::get_branch,
};
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn list(pkg_path: &Path, bin_path: &Path, query: &[String], color: bool, details: bool) {
    let working_dir = current_dir().unwrap_or(PathBuf::new());
    for repo in get_repos(pkg_path, pkg_path, query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            let mut output = String::new();

            if color && working_dir == repo {
                output.push_str(&format!("\u{1b}[1m{}\u{1b}[m", rel_path.display()));
            } else {
                output.push_str(&rel_path.to_string_lossy());
            }

            if let Some(branch) = get_branch(&repo) {
                if &branch != "master" {
                    if color {
                        output.push_str(&format!(" \u{1b}[93m@{}\u{1b}[m", branch));
                    } else {
                        output.push_str(&format!(" @{}", branch));
                    }
                }
            }

            let exes = get_repo_exes(&repo);

            if !details && exes.len() > 0 {
                let mut linked_exes_count = 0;
                for exe in &exes {
                    if get_links_to(&exe, bin_path).len() > 0 {
                        linked_exes_count += 1;
                    }
                }

                if color {
                    output.push_str(&format!(" \u{1b}[96m<{}>\u{1b}[m", linked_exes_count));
                } else {
                    output.push_str(&format!(" <{}>", linked_exes_count));
                }
            }

            output.push_str("\n");

            if details {
                for exe in exes {
                    if let Some(exe_name_os) = exe.file_name() {
                        let exe_name;
                        if let Ok(rel_exe) = exe.strip_prefix(&repo) {
                            exe_name = rel_exe.to_string_lossy();
                        } else {
                            exe_name = exe_name_os.to_string_lossy();
                        }
                        let links = get_links_to(&exe, bin_path);

                        if links.len() == 0 {
                            if color {
                                output.push_str(&format!("  \u{1b}[2m{}\u{1b}[m\n", exe_name));
                            } else {
                                output.push_str(&format!("  {}\n", exe_name));
                            }
                        } else {
                            let mut link_list = String::new();
                            for link in links {
                                if let Some(link_name_os) = link.file_name() {
                                    let link_name = link_name_os.to_string_lossy();
                                    if !link_list.is_empty() {
                                        link_list.push_str(" ");
                                    }
                                    link_list.push_str(&link_name);
                                }
                            }
                            if color {
                                output.push_str(&format!(
                                    "  {} \u{1b}[2m<-\u{1b}[m \u{1b}[96m{}\u{1b}[m\n",
                                    exe_name, link_list
                                ));
                            } else {
                                output.push_str(&format!("  {} <- {}\n", exe_name, link_list));
                            }
                        }
                    }
                }
            }

            print!("{}", output);
        }
    }
}
