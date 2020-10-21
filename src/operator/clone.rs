use crate::prompt::{prompt_no, Answer};
use crate::query::{repo_id_from_url, url_from_query};
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

pub fn clone(pkg_path: &Path, query: &[String], answer: &Answer) -> Result<(), String> {
    if query.len() <= 0 {
        Err(format!("query required"))
    } else {
        let mut cloned_ids: Vec<String> = Vec::new();
        for q in query {
            if let Some(url) = url_from_query(&q) {
                let repo_id = repo_id_from_url(&url).unwrap();
                let repo_path = pkg_path.join(&repo_id);
                if !repo_path.exists()
                    || prompt_no(&format!("{} exists. overwrite?", repo_id), answer)
                {
                    if !repo_path.exists() || remove_dir_all(&repo_path).is_ok() {
                        match Command::new("git")
                            .args(&["clone", &url, &repo_path.to_string_lossy()])
                            .status()
                        {
                            Ok(result) => {
                                if result.success() {
                                    cloned_ids.push(String::from(repo_id));
                                } else {
                                    println!("git clone failed");
                                }
                            }
                            Err(msg) => println!("{}", msg),
                        }
                    } else {
                        println!("failed to remove {}", repo_id);
                    }
                }
            } else {
                println!("could not build url from query {}", q);
            }
        }
        println!("{} packages cloned", cloned_ids.len());
        Ok(())
    }
}
