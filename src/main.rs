use dirs::home_dir;
use std::collections::HashMap;
use std::env::args;

mod output;
use output::print_usage;

mod args;
use args::parse_args;

mod operator;
use operator::{cat, clone, dir, list, make, remove, symlink, update};

fn main() {
    match parse_args(
        &args().collect::<Vec<String>>()[1..],
        &[
            ("h", "help"),
            ("c", "color"),
            ("f", "force"),
            ("help", "help"),
            ("color", "color"),
            ("force", "force"),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<&str, &str>>(),
        &HashMap::new(),
    ) {
        Err(msg) => println!("{}", msg),
        Ok((words, flags, _)) => {
            if flags.contains(&String::from("help")) {
                print_usage(flags.contains(&String::from("color")));
            } else {
                if let Some(home) = home_dir() {
                    let osoy_path = home.join(".osoy");
                    if osoy_path.is_dir() {
                        match words.get(0) {
                            Some(operator) => match operator.as_str() {
                                "l" | "list" => list(
                                    &osoy_path.join("packages"),
                                    &osoy_path.join("bin"),
                                    &words[1..],
                                    flags.contains(&String::from("color")),
                                ),
                                "c" | "clone" => clone(
                                    &osoy_path.join("packages"),
                                    &osoy_path.join("bin"),
                                    &words[1..],
                                    flags.contains(&String::from("force")),
                                ),
                                "r" | "remove" => remove(
                                    &osoy_path.join("packages"),
                                    &osoy_path.join("bin"),
                                    &words[1..],
                                    flags.contains(&String::from("force")),
                                ),
                                "s" | "symlink" => symlink(
                                    &osoy_path.join("packages"),
                                    &osoy_path.join("bin"),
                                    &words[1..],
                                    flags.contains(&String::from("force")),
                                ),
                                "u" | "update" => update(
                                    &osoy_path.join("packages"),
                                    &osoy_path.join("bin"),
                                    &words[1..],
                                    flags.contains(&String::from("force")),
                                ),
                                "m" | "make" => make(
                                    &osoy_path.join("packages"),
                                    &osoy_path.join("bin"),
                                    &words[1..],
                                    flags.contains(&String::from("force")),
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
                            None => print_usage(flags.contains(&String::from("color"))),
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
