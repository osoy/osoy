use std::env::args;
use std::fs::read_dir;
use std::path::Path;

use dirs::home_dir;

mod usage;
use usage::{error, msg, print_usage};

fn list(dir: &Path) {
    let mut git_dir = dir.to_path_buf();
    git_dir.push(".git");
    if git_dir.is_dir() {
        if let Some(path) = dir.to_str() {
            println!("{}", path);
        }
    } else {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        list(entry_path.as_path());
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if let Some(home) = home_dir() {
        let mut osoy_dir = home.to_path_buf();
        osoy_dir.push(".osoy");
        match args.get(1) {
            Some(operator) => match operator.as_str() {
                "l" | "list" => {
                    let mut dir = osoy_dir.to_path_buf();
                    dir.push("packages");
                    list(dir.as_path())
                }
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
        error("home directory not found")
    }
}
