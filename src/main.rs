use dirs::home_dir;
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
use operator::*;

fn main() {
    match parse_args(
        &args().collect::<Vec<String>>()[1..],
        &[
            &["color", "c"],
            &["help", "h"],
            &["version", "v"],
            &["quiet", "q"],
            &["force", "f"],
            &["defaults", "y"],
            &["deny", "n"],
        ],
        &[&["option", "o"]],
    ) {
        Err(msg) => println!("{}", msg),
        Ok(parsed) => {
            if parsed.flags.contains(&"help") {
                print_usage(parsed.flags.contains(&"color"));
            } else if parsed.flags.contains(&"version") {
                println!("{}", env!("CARGO_PKG_VERSION"));
            } else {
                if let Some(home) = home_dir() {
                    let osoy_path = home.join(".osoy");
                    let packages_dir = osoy_path.join("packages");
                    let bin_dir = osoy_path.join("bin");
                    create_dir_if_absent(&packages_dir);
                    create_dir_if_absent(&bin_dir);

                    let color = parsed.flags.contains(&"color");
                    let quiet = parsed.flags.contains(&"quiet");
                    let option = parsed.options.get("option");

                    match Answer::new(
                        parsed.flags.contains(&"force"),
                        parsed.flags.contains(&"defaults"),
                        parsed.flags.contains(&"deny"),
                    ) {
                        Ok(answer) => match parsed.words.get(0) {
                            Some(operator) => {
                                let operands = &parsed.words[1..];
                                match operator.as_str() {
                                    "l" | "list" => {
                                        list(&packages_dir, &bin_dir, operands, color, quiet)
                                    }
                                    "s" | "status" => status(&packages_dir, operands, color, quiet),
                                    "c" | "clone" => {
                                        clone(&packages_dir, &bin_dir, operands, &answer, &option)
                                    }
                                    "f" | "fork" => {
                                        fork(&packages_dir, &bin_dir, operands, &answer, &option)
                                    }
                                    "r" | "remove" => {
                                        remove(&packages_dir, &bin_dir, operands, &answer)
                                    }
                                    "n" | "new" => new(&packages_dir, operands, &answer),
                                    "y" | "symlink" => {
                                        symlink(&packages_dir, &bin_dir, operands, &answer)
                                    }
                                    "u" | "update" => {
                                        update(&packages_dir, &bin_dir, operands, &answer, &option)
                                    }
                                    "b" | "build" => {
                                        build(&packages_dir, &bin_dir, operands, &answer, &option)
                                    }
                                    "m" | "move" => {
                                        relocate(&packages_dir, &bin_dir, operands, &answer)
                                    }
                                    "dir" => dir(&packages_dir, operands),
                                    "readme" => {
                                        cat(&packages_dir, operands, "(README|readme)(.md)?")
                                    }
                                    "license" => {
                                        cat(&packages_dir, operands, "(LICENSE|license)(.md)?")
                                    }
                                    _ => println!("unknown operator '{}'", operator),
                                };
                            }
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
