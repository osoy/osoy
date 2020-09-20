use crate::operator::{build, symlink};
use crate::prompt::{prompt_no, prompt_yes, Answer};
use crate::query::{build::get_build_method, repo_id_from_url, url_from_query};
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

pub fn clone(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&Vec<String>>,
) {
    if query.len() <= 0 {
        println!("query required");
    } else {
        let mut cloned_ids: Vec<String> = Vec::new();
        let mut have_makefiles = false;
        for q in query {
            if let Some(url) = url_from_query(&q) {
                let repo_id = repo_id_from_url(&url).unwrap();
                let repo_path = pkg_path.join(&repo_id);
                if !repo_path.exists()
                    || prompt_no(&format!("package '{}' exists. overwrite?", repo_id), answer)
                {
                    if !repo_path.exists() || remove_dir_all(&repo_path).is_ok() {
                        match Command::new("git")
                            .args(&["clone", &url, &repo_path.to_string_lossy()])
                            .status()
                        {
                            Ok(result) => {
                                if result.success() {
                                    cloned_ids.push(String::from(repo_id));
                                    if !have_makefiles {
                                        have_makefiles = get_build_method(&repo_path).is_some()
                                    }
                                } else {
                                    println!("git clone failed");
                                }
                            }
                            Err(msg) => println!("git clone failed to start '{}'", msg),
                        }
                    } else {
                        println!("failed to remove package '{}'", repo_id);
                    }
                }
            } else {
                println!("couldn't build url from query '{}'", q);
            }
        }
        println!("{} packages cloned", cloned_ids.len());
        if have_makefiles && prompt_yes("build cloned packages?", answer) {
            build(pkg_path, bin_path, &cloned_ids, answer, option);
        } else {
            symlink(pkg_path, bin_path, &cloned_ids, answer);
        }
    }
}
