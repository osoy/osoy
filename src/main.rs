use dirs::home_dir;
use std::env::args;
use std::path::Path;

mod usage;
use usage::{error, msg, print_usage};

mod query;
use query::{get_exes, get_links_to, get_repos};

fn list(pkg_path: &Path, bin_path: &Path) {
    for repo in get_repos(pkg_path) {
        if let Ok(rel_path) = repo.strip_prefix(pkg_path) {
            if let Some(repo_id) = rel_path.to_str() {
                println!("{}", repo_id);
            }
        }
        for exe in get_exes(repo.as_path()) {
            if let Some(filename_os) = exe.file_name() {
                if let Some(filename) = filename_os.to_str() {
                    println!(
                        "  {} ({})",
                        filename,
                        get_links_to(exe.as_path(), bin_path).len()
                    );
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if let Some(home) = home_dir() {
        let osoy_path = home.join(".osoy");
        if osoy_path.is_dir() {
            match args.get(1) {
                Some(operator) => match operator.as_str() {
                    "l" | "list" => list(
                        osoy_path.join("packages").as_path(),
                        osoy_path.join("bin").as_path(),
                    ),
                    "c" | "clone" => msg("to be implemented: clone <query>"),
                    "r" | "remove" => msg("to be implemented: remove <query>"),
                    "s" | "symlink" => msg("to be implemented: symlink [query]"),
                    "u" | "update" => msg("to be implemented: update [query]"),
                    "m" | "make" => msg("to be implemented: make [query]"),
                    "dir" => msg("to be implemented: dir <query>"),
                    "read" => msg("to be implemented: read <query>"),
                    "license" => msg("to be implemented: license <query>"),
                    _ => error(&format!("unknown operator '{}'", operator)),
                },
                None => print_usage(),
            }
        } else {
            error("osoy directory not found")
        }
    } else {
        error("home directory not found")
    }
}
