use crate::prompt::{prompt_no, Answer};
use crate::query::{create_symlink, get_links_to, get_repo_exes, get_repos, remove_orphan_links};
use std::fs::remove_file;
use std::path::Path;

pub fn symlink(pkg_path: &Path, bin_path: &Path, query: &[String], answer: &Answer) {
    remove_orphan_links(bin_path);
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            for exe in get_repo_exes(&repo) {
                if let Some(filename_os) = exe.file_name() {
                    if let Some(filename) = filename_os.to_str() {
                        if get_links_to(&exe, bin_path).len() == 0 {
                            let link = bin_path.join(filename);
                            if !link.exists()
                                || prompt_no(
                                    &format!(
                                        "node '{}' exists. overwrite pointing to '{}'?",
                                        link.display(),
                                        if let Ok(rel_path) = exe.strip_prefix(pkg_path) {
                                            rel_path.display()
                                        } else {
                                            exe.display()
                                        }
                                    ),
                                    answer,
                                )
                            {
                                if !link.exists() || remove_file(&link).is_ok() {
                                    match create_symlink(&exe, &link) {
                                        Ok(_) => {
                                            count += 1;
                                            println!(
                                                "{} -> {}",
                                                filename,
                                                if let Ok(rel_path) = exe.strip_prefix(pkg_path) {
                                                    rel_path.display()
                                                } else {
                                                    exe.display()
                                                }
                                            );
                                        }
                                        Err(msg) => println!("failed to create a link\n{}", msg),
                                    }
                                } else {
                                    println!("failed to remove '{}'", link.display());
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("{} links created", count);
    }
}
