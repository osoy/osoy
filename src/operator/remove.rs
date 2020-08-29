use crate::prompt::{prompt_yes, Answer};
use crate::query::{get_repos, remove_orphan_links, remove_rec_if_empty};
use std::fs::remove_dir_all;
use std::path::Path;

pub fn remove(pkg_path: &Path, bin_path: &Path, query: &[String], answer: &Answer) {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);

    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        println!("removing following packages:");
        for repo in &repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                println!("{}", rel_path.display());
            }
        }

        if prompt_yes("proceed?", answer) {
            for mut repo in repos {
                if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                    match remove_dir_all(&repo) {
                        Ok(_) => {
                            count += 1;
                            println!("package '{}' removed", rel_path.display());
                            repo.pop();
                            remove_rec_if_empty(&repo);
                        }
                        Err(msg) => {
                            println!("failed to remove package '{}'\n{}", rel_path.display(), msg)
                        }
                    };
                }
            }
            remove_orphan_links(bin_path);
            println!("{} packages removed", count);
        }
    }
}
