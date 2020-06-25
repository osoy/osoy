use dirs::home_dir;
use std::collections::HashMap;
use std::env::args;

mod output;
use output::print_usage;

mod args;
use args::parse_args;

mod operator;
use operator::{cat, clone, dir, fork, list, make, remove, symlink, update, Answer};

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
                    if osoy_path.is_dir() {
                        let color = flags.contains(&"color");
                        let option = options.get("option");
                        match Answer::new(
                            flags.contains(&"force"),
                            flags.contains(&"defaults"),
                            flags.contains(&"deny"),
                        ) {
                            Ok(answer) => match words.get(0) {
                                Some(operator) => match operator.as_str() {
                                    "l" | "list" => list(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        color,
                                    ),
                                    "c" | "clone" => clone(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        &answer,
                                        &option,
                                    ),
                                    "f" | "fork" => fork(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        &answer,
                                        &option,
                                    ),
                                    "r" | "remove" => remove(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        &answer,
                                    ),
                                    "s" | "symlink" => symlink(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        &answer,
                                    ),
                                    "u" | "update" => update(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        &answer,
                                        &option,
                                    ),
                                    "m" | "make" => make(
                                        &osoy_path.join("packages"),
                                        &osoy_path.join("bin"),
                                        &words[1..],
                                        &answer,
                                        &option,
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
                                None => print_usage(color),
                            },
                            Err(msg) => println!("{}", msg),
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
}
