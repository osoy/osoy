use regex::Regex;
use std::env::set_current_dir;
use std::fs::{remove_dir_all, remove_file, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

mod query;
use query::{
    create_symlink, get_exes, get_first_file, get_links_to, get_orphan_links, get_repos,
    url_from_query,
};

mod prompt;
use prompt::{prompt_no, prompt_yes};

pub fn list(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    for repo in get_repos(pkg_path, pkg_path, query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            println!("{}", rel_path.display());
        }
        for exe in get_exes(&repo) {
            if let Some(filename_os) = exe.file_name() {
                let filename = filename_os.to_string_lossy();
                let links = get_links_to(&exe, bin_path);
                if links.len() == 0 {
                    println!("  {}", filename);
                } else {
                    let mut link_list = String::new();
                    for link in links {
                        if let Some(filename_os) = link.file_name() {
                            let filename = filename_os.to_string_lossy();
                            if !link_list.is_empty() {
                                link_list.push_str(", ");
                            }
                            link_list.push_str(&filename);
                        }
                    }
                    println!("  {} <- {}", filename, link_list);
                }
            }
        }
    }
}

fn remove_orphan_links(bin_path: &Path) {
    let mut count = 0;
    for link in get_orphan_links(bin_path) {
        if remove_file(&link).is_ok() {
            count += 1;
        }
    }
    println!("{} links removed", count);
}

pub fn remove(pkg_path: &Path, bin_path: &Path, query: &[String]) {
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
        if prompt_yes("proceed?") {
            for repo in repos {
                if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                    match remove_dir_all(&repo) {
                        Ok(_) => {
                            count += 1;
                            println!("package '{}' removed", rel_path.display());
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

pub fn symlink(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    remove_orphan_links(bin_path);
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            for exe in get_exes(&repo) {
                if let Some(filename_os) = exe.file_name() {
                    if let Some(filename) = filename_os.to_str() {
                        if get_links_to(&exe, bin_path).len() == 0 {
                            let link = bin_path.join(filename);
                            if !link.exists()
                                || prompt_no(&format!(
                                    "node '{}' exists. overwrite pointing to '{}'?",
                                    link.display(),
                                    if let Ok(rel_path) = exe.strip_prefix(pkg_path) {
                                        rel_path.display()
                                    } else {
                                        exe.display()
                                    }
                                ))
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

pub fn clone(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    let mut cloned_ids: Vec<String> = Vec::new();
    if query.len() <= 0 {
        println!("query required");
    } else {
        for q in query {
            if let Some(url) = url_from_query(&q) {
                let repo_id = Regex::new("^.*://").unwrap().replace(&url, "");
                let repo_path = pkg_path.join(repo_id.as_ref());
                if !repo_path.exists()
                    || prompt_no(&format!("package '{}' exists. overwrite?", repo_id))
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
        symlink(pkg_path, bin_path, &cloned_ids);
        println!("{} packages cloned", cloned_ids.len());
    }
}

pub fn update(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                println!("{}", rel_path.display());
                if set_current_dir(&repo).is_ok() {
                    match Command::new("git").arg("pull").status() {
                        Ok(result) => {
                            if result.success() {
                                count += 1;
                            } else {
                                println!("git pull failed");
                            }
                        }
                        Err(msg) => println!("git pull failed to start '{}'", msg),
                    }
                } else {
                    println!("failed to access '{}'", repo.display());
                }
            }
        }
        symlink(pkg_path, bin_path, query);
        println!("{} packages updated", count);
    }
}

pub fn make(pkg_path: &Path, bin_path: &Path, query: &[String]) {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                if set_current_dir(&repo).is_ok() {
                    if repo.join("Makefile").is_file() || repo.join("makefile").is_file() {
                        println!("{}", rel_path.display());
                        match Command::new("make").status() {
                            Ok(result) => {
                                if result.success() {
                                    count += 1;
                                } else {
                                    println!("make failed");
                                }
                            }
                            Err(msg) => println!("make failed '{}'", msg),
                        }
                    }
                } else {
                    println!("failed to access '{}'", repo.display());
                }
            }
        }
        symlink(pkg_path, bin_path, query);
        println!("{} packages updated", count);
    }
}

pub fn dir(pkg_path: &Path, query: &[String]) {
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else if repos.len() > 1 {
        println!("query is ambigious '{}'", query.join(" "));
    } else {
        println!("{}", repos[0].display());
    }
}

pub fn cat(pkg_path: &Path, query: &[String], file_re: &str) {
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else if repos.len() > 1 {
        println!("query is ambigious '{}'", query.join(" "));
    } else {
        let repo = &repos[0];
        if let Some(file) = get_first_file(&repo, file_re) {
            let mut buffer = String::new();
            if let Ok(mut f) = File::open(&file) {
                if f.read_to_string(&mut buffer).is_ok() {
                    println!("{}", buffer);
                } else {
                    println!("couldn't read '{}'", file.display());
                }
            } else {
                println!("couldn't open '{}'", file.display());
            }
        } else {
            println!("no file found");
        }
    }
}
