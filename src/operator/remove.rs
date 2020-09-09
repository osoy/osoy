use crate::prompt::{prompt, prompt_yes, Answer};
use crate::query::{get_repos, remove_orphan_links, remove_rec_if_empty};
use std::fs::remove_dir_all;
use std::path::Path;

pub fn remove(pkg_path: &Path, bin_path: &Path, query: &[String], answer: &Answer) {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    let remove_count = repos.len();

    if remove_count <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        println!("removing following packages ({}):", remove_count);

        for repo in &repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                println!("{}", rel_path.display());
            }
        }

        if remove_count == 1 && prompt_yes("proceed?", answer)
            || remove_count > 1 && prompt(&format!("remove {} packages?", remove_count), answer)
        {
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
