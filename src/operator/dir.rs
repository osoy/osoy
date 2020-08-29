use crate::query::get_repos;
use std::path::Path;

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
