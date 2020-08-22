use dirs::home_dir;
use std::collections::HashMap;
use std::env::args;

pub mod query;
use query::create_dir_if_absent;

pub mod prompt;
use prompt::Answer;

mod output;
use output::print_usage;

mod args;
use args::parse_args;

mod operator;
use operator::{cat, clone, dir, fork, list, make, new, remove, status, symlink, update};

fn main() {
    match parse_args(
        &args().collect::<Vec<String>>()[1..],
        [
            ("h", "help"),
            ("c", "color"),
            ("v", "version"),
            ("help", "help"),
            ("color", "color"),
            ("version", "version"),
            ("f", "force"),
            ("y", "defaults"),
            ("n", "deny"),
            ("force", "force"),
            ("defaults", "defaults"),
            ("deny", "deny"),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<&str, &str>>(),
        [("o", "option"), ("option", "option")]
            .iter()
            .cloned()
            .collect::<HashMap<&str, &str>>(),
    ) {
        Err(msg) => println!("{}", msg),
        Ok((words, flags, options)) => {
            if flags.contains(&"help") {
                print_usage(flags.contains(&"color"));
            } else if flags.contains(&"version") {
                println!("{}", env!("CARGO_PKG_VERSION"));
            } else {
                if let Some(home) = home_dir() {
                    let osoy_path = home.join(".osoy");
                    let packages_dir = osoy_path.join("packages");
                    let bin_dir = osoy_path.join("bin");
                    create_dir_if_absent(&packages_dir);
                    create_dir_if_absent(&bin_dir);
                    let color = flags.contains(&"color");
                    let option = options.get("option");
                    match Answer::new(
                        flags.contains(&"force"),
                        flags.contains(&"defaults"),
                        flags.contains(&"deny"),
                    ) {
                        Ok(answer) => match words.get(0) {
                            Some(operator) => match operator.as_str() {
                                "l" | "list" => list(&packages_dir, &bin_dir, &words[1..], color),
                                "s" | "status" => status(&packages_dir, &words[1..]),
                                "c" | "clone" => {
                                    clone(&packages_dir, &bin_dir, &words[1..], &answer, &option)
                                }
                                "f" | "fork" => {
                                    fork(&packages_dir, &bin_dir, &words[1..], &answer, &option)
                                }
                                "r" | "remove" => {
                                    remove(&packages_dir, &bin_dir, &words[1..], &answer)
                                }
                                "n" | "new" => new(&packages_dir, &words[1..], &answer),
                                "y" | "symlink" => {
                                    symlink(&packages_dir, &bin_dir, &words[1..], &answer)
                                }
                                "u" | "update" => {
                                    update(&packages_dir, &bin_dir, &words[1..], &answer, &option)
                                }
                                "m" | "make" => {
                                    make(&packages_dir, &bin_dir, &words[1..], &answer, &option)
                                }
                                "dir" => dir(&packages_dir, &words[1..]),
                                "readme" => {
                                    cat(&packages_dir, &words[1..], "(README|readme)(.md)?")
                                }
                                "license" => {
                                    cat(&packages_dir, &words[1..], "(LICENSE|license)(.md)?")
                                }
                                _ => println!("unknown operator '{}'", operator),
                            },
                            None => print_usage(color),
                        },
                        Err(msg) => println!("{}", msg),
                    }
                } else {
                    println!("home directory not found")
                }
            }
        }
    }
}
