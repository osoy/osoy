use crate::query::{get_branch, get_exes, get_links_to, get_repos};
use std::path::Path;

pub fn list(pkg_path: &Path, bin_path: &Path, query: &[String], color: bool, quiet: bool) {
    for repo in get_repos(pkg_path, pkg_path, query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            print!("{}", rel_path.display());

            let exes = get_exes(&repo);

            if let Some(branch) = get_branch(&repo) {
                if &branch != "master" {
                    if color {
                        print!(" \u{1b}[93m@{}\u{1b}[m", branch);
                    } else {
                        print!(" @{}", branch);
                    }
                }
            }

            if quiet && exes.len() > 0 {
                let mut linked_exes_count = 0;
                for exe in &exes {
                    if get_links_to(&exe, bin_path).len() > 0 {
                        linked_exes_count += 1;
                    }
                }

                if color {
                    print!(" \u{1b}[96m<{}>\u{1b}[m", linked_exes_count);
                } else {
                    print!(" <{}>", linked_exes_count);
                }
            }

            println!();

            if !quiet {
                for exe in exes {
                    if let Some(filename_os) = exe.file_name() {
                        let filename = filename_os.to_string_lossy();
                        let links = get_links_to(&exe, bin_path);

                        if links.len() == 0 {
                            if color {
                                println!("  \u{1b}[2m{}\u{1b}[m", filename);
                            } else {
                                println!("  {}", filename);
                            }
                        } else {
                            let mut link_list = String::new();
                            for link in links {
                                if let Some(filename_os) = link.file_name() {
                                    let filename = filename_os.to_string_lossy();
                                    if !link_list.is_empty() {
                                        link_list.push_str(" ");
                                    }
                                    link_list.push_str(&filename);
                                }
                            }
                            if color {
                                println!(
                                    "  {} \u{1b}[2m<-\u{1b}[m \u{1b}[96m{}\u{1b}[m",
                                    filename, link_list
                                );
                            } else {
                                println!("  {} <- {}", filename, link_list);
                            }
                        }
                    }
                }
            }
        }
    }
}
