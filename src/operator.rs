use regex::Regex;
use std::env::set_current_dir;
use std::fs::{remove_dir_all, remove_file, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

mod query;
use query::{
    create_symlink, get_branch, get_exes, get_first_file, get_links_to, get_repos, has_makefile,
    remove_orphan_links, remove_rec_if_empty, url_from_query,
};

mod prompt;
pub use prompt::Answer;
use prompt::{prompt_no, prompt_yes};

pub fn list(pkg_path: &Path, bin_path: &Path, query: &[String], color: bool) {
    for repo in get_repos(pkg_path, pkg_path, query) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            print!("{}", rel_path.display());
            if let Some(branch) = get_branch(&repo) {
                if &branch != "master" {
                    if color {
                        print!(" \u{1b}[1m\u{1b}[33m{}\u{1b}[0m", branch);
                    } else {
                        print!(" {}", branch);
                    }
                }
            }
            println!();

            for exe in get_exes(&repo) {
                if let Some(filename_os) = exe.file_name() {
                    let filename = filename_os.to_string_lossy();
                    let links = get_links_to(&exe, bin_path);

                    if links.len() == 0 {
                        if color {
                            println!("  \u{1b}[2m{}\u{1b}[0m", filename);
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
                                "  {} \u{1b}[2m<-\u{1b}[0m \u{1b}[1m\u{1b}[36m{}\u{1b}[0m",
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

pub fn symlink(pkg_path: &Path, bin_path: &Path, query: &[String], answer: &Answer) {
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

pub fn make(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    let mut count = 0;
    let repos = get_repos(pkg_path, pkg_path, query);
    if repos.len() <= 0 {
        println!("no packages satisfy query '{}'", query.join(" "));
    } else {
        for repo in repos {
            if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
                if set_current_dir(&repo).is_ok() {
                    if has_makefile(&repo) {
                        println!("{}", rel_path.display());
                        let mut cmd = Command::new("make");
                        if let Some(option) = option {
                            cmd.arg(option);
                            println!("> make {}", option);
                        } else {
                            println!("> make");
                        }
                        match cmd.status() {
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
        println!("{} packages built", count);
        symlink(pkg_path, bin_path, query, answer);
    }
}

pub fn clone(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    if query.len() <= 0 {
        println!("query required");
    } else {
        let mut cloned_ids: Vec<String> = Vec::new();
        let mut have_makefiles = false;
        for q in query {
            if let Some(url) = url_from_query(&q) {
                let repo_id = Regex::new("^.*://")
                    .unwrap()
                    .replace(&url, "")
                    .to_lowercase();
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
                                        have_makefiles = has_makefile(&repo_path)
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
        if have_makefiles && prompt_yes("make cloned packages?", answer) {
            make(pkg_path, bin_path, &cloned_ids, answer, option);
        } else {
            symlink(pkg_path, bin_path, &cloned_ids, answer);
        }
    }
}

pub fn fork(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    if query.len() <= 0 {
        println!("query and fork destination required");
    } else if query.len() <= 1 {
        println!("fork url required");
    } else {
        let q = &query[0];
        let fork_dest = &query[1];
        if let Some(url) = url_from_query(&q) {
            if let Some(fork_url) = url_from_query(&fork_dest) {
                let repo_id = Regex::new("^.*://")
                    .unwrap()
                    .replace(&fork_url, "")
                    .to_lowercase();
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
                                    if set_current_dir(&repo_path).is_ok() {
                                        println!("package cloned from '{}'", url);
                                        match Command::new("git")
                                            .args(&["remote", "rename", "origin", "upstream"])
                                            .status()
                                        {
                                            Ok(_) => {}
                                            Err(msg) => println!("error: {}", msg),
                                        }
                                        match Command::new("git")
                                            .args(&["remote", "add", "origin", &fork_url])
                                            .status()
                                        {
                                            Ok(result) => {
                                                if result.success() {
                                                    println!("added remote origin '{}'", fork_url);
                                                }
                                            }
                                            Err(msg) => println!("error: {}", msg),
                                        }
                                    }
                                    if has_makefile(&repo_path)
                                        && prompt_yes("make forked package?", answer)
                                    {
                                        make(
                                            pkg_path,
                                            bin_path,
                                            &[repo_id.clone()],
                                            answer,
                                            option,
                                        );
                                    } else {
                                        symlink(pkg_path, bin_path, &[repo_id.clone()], answer);
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
                println!("couldn't build url from fork destination '{}'", fork_dest);
            }
        } else {
            println!("couldn't build url from query '{}'", q);
        }
    }
}

pub fn update(
    pkg_path: &Path,
    bin_path: &Path,
    query: &[String],
    answer: &Answer,
    option: &Option<&String>,
) {
    let mut cloned_ids: Vec<String> = Vec::new();
    let mut have_makefiles = false;
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
                                cloned_ids.push(rel_path.to_string_lossy().to_string());
                                if !have_makefiles {
                                    have_makefiles = has_makefile(&repo)
                                }
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
        println!("{} packages updated", &cloned_ids.len());
        if have_makefiles && prompt_yes("make updated packages?", answer) {
            make(pkg_path, bin_path, &cloned_ids, answer, option);
        } else {
            symlink(pkg_path, bin_path, &cloned_ids, answer);
        }
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
