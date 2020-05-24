use dirs::home_dir;
use std::env::args;

mod output;
use output::{error, msg, print_usage};

mod args;
use args::parse_args;

mod operator;
use operator::{clone, list, remove, symlink};

fn main() {
    match parse_args(&args().collect::<Vec<String>>()[1..], &["c"], &[]) {
        Err(msg) => error(&msg),
        Ok((words, _flags, _opts)) => {
            if let Some(home) = home_dir() {
                let osoy_path = home.join(".osoy");
                if osoy_path.is_dir() {
                    match words.get(0) {
                        Some(operator) => match operator.as_str() {
                            "l" | "list" => list(
                                &osoy_path.join("packages"),
                                &osoy_path.join("bin"),
                                &words[1..],
                            ),
                            "c" | "clone" => clone(&osoy_path.join("packages"), &words[1..]),
                            "r" | "remove" => remove(
                                &osoy_path.join("packages"),
                                &osoy_path.join("bin"),
                                &words[1..],
                            ),
                            "s" | "symlink" => symlink(
                                &osoy_path.join("packages"),
                                &osoy_path.join("bin"),
                                &words[1..],
                            ),
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
    }
}
