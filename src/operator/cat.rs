use crate::query::{get_first_file, get_repos};
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
