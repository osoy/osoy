use std::env::args;

pub mod query;

pub mod prompt;
use prompt::Answer;

mod output;
use output::print_usage;

mod args;
use args::parse_args;

mod datadir;
use datadir::data_dirs;

mod operator;
use operator::*;

fn main() {
    match parse_args(
        &args().collect::<Vec<String>>()[1..],
        &[
            &["color", "c"],
            &["help", "h"],
            &["version", "v"],
            &["details", "d"],
            &["force", "f"],
            &["defaults", "y"],
            &["deny", "n"],
        ],
        &[&["option", "o"]],
    ) {
        Err(msg) => eprintln!("{}", msg),
        Ok(parsed) => {
            let color = parsed.flags.contains(&"color");
            let details = parsed.flags.contains(&"details");
            let option = parsed.options.get("option");

            if parsed.flags.contains(&"help") {
                print_usage();
            } else if parsed.flags.contains(&"version") {
                println!(
                    "{} version {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                );
            } else {
                match data_dirs() {
                    Err(msg) => eprintln!("{}", msg),
                    Ok(data) => match Answer::new(
                        parsed.flags.contains(&"force"),
                        parsed.flags.contains(&"defaults"),
                        parsed.flags.contains(&"deny"),
                    ) {
                        Err(msg) => eprintln!("{}", msg),
                        Ok(answer) => match parsed.operator {
                            Some(operator) => {
                                match match operator.as_str() {
                                    "n" | "new" => new(&data.packages, &parsed.operands, &answer),
                                    "cl" | "clone" => {
                                        clone(&data.packages, &parsed.operands, &answer)
                                    }
                                    "fork" => fork(&data.packages, &parsed.operands, &answer),
                                    "pull" => update(&data.packages, &parsed.operands),
                                    "ln" | "link" => symlink(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                    ),
                                    "ls" | "list" => list(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        color,
                                        details,
                                    ),
                                    "rm" | "remove" => {
                                        remove(&data.packages, &data.bin, &parsed.operands, &answer)
                                    }
                                    "mv" | "move" => {
                                        relocate(&data.packages, &parsed.operands, &answer)
                                    }
                                    "st" | "status" => {
                                        status(&data.packages, &parsed.operands, color, details)
                                    }
                                    "mk" | "make" => {
                                        build(&data.packages, &parsed.operands, &option)
                                    }
                                    "dir" => dir(&data.packages, &parsed.operands),
                                    "readme" => cat(
                                        &data.packages,
                                        &parsed.operands,
                                        "(README|readme)(.md)?",
                                    ),
                                    "license" => cat(
                                        &data.packages,
                                        &parsed.operands,
                                        "(LICENSE|license)(.md)?",
                                    ),
                                    _ => Err(format!("unknown operator '{}'", operator)),
                                } {
                                    Err(msg) => eprintln!("{}", msg),
                                    Ok(_) => {}
                                }
                            }
                            None => print_usage(),
                        },
                    },
                }
            }
        }
    }
}
