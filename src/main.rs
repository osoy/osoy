use dirs::home_dir;
use std::env::args;

mod output;
use output::print_usage;

mod args;
use args::parse_args;

mod operator;
use operator::{cat, clone, dir, list, make, remove, symlink, update};

fn main() {
    match parse_args(&args().collect::<Vec<String>>()[1..], &["c"], &[]) {
        Err(msg) => println!("{}", msg),
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
                            "c" | "clone" => clone(
                                &osoy_path.join("packages"),
                                &osoy_path.join("bin"),
                                &words[1..],
                            ),
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
                            "u" | "update" => update(
                                &osoy_path.join("packages"),
                                &osoy_path.join("bin"),
                                &words[1..],
                            ),
                            "m" | "make" => make(
                                &osoy_path.join("packages"),
                                &osoy_path.join("bin"),
                                &words[1..],
                            ),
                            "dir" => dir(&osoy_path.join("packages"), &words[1..]),
                            "readme" => cat(
                                &osoy_path.join("packages"),
                                &words[1..],
                                "(README|readme)(.md)?",
                            ),
                            "license" => cat(
                                &osoy_path.join("packages"),
                                &words[1..],
                                "(LICENSE|license)(.md)?",
                            ),
                            _ => println!("unknown operator '{}'", operator),
                        },
                        None => print_usage(),
                    }
                } else {
                    println!("osoy directory not found")
                }
            } else {
                println!("home directory not found")
            }
        }
    }
}
