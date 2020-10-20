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
        Err(msg) => println!("{}", msg),
        Ok(parsed) => {
            let color = parsed.flags.contains(&"color");
            let details = parsed.flags.contains(&"details");
            let option = parsed.options.get("option");

            if parsed.flags.contains(&"help") {
                print_usage(color);
            } else if parsed.flags.contains(&"version") {
                println!("{}", env!("CARGO_PKG_VERSION"));
            } else {
                match data_dirs() {
                    Err(msg) => println!("{}", msg),
                    Ok(data) => match Answer::new(
                        parsed.flags.contains(&"force"),
                        parsed.flags.contains(&"defaults"),
                        parsed.flags.contains(&"deny"),
                    ) {
                        Err(msg) => println!("{}", msg),
                        Ok(answer) => match parsed.operator {
                            Some(operator) => {
                                match operator.as_str() {
                                    "l" | "list" => list(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        color,
                                        details,
                                    ),
                                    "s" | "status" => {
                                        status(&data.packages, &parsed.operands, color, details)
                                    }
                                    "c" | "clone" => clone(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                        &option,
                                    ),
                                    "f" | "fork" => fork(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                        &option,
                                    ),
                                    "r" | "remove" => {
                                        remove(&data.packages, &data.bin, &parsed.operands, &answer)
                                    }
                                    "n" | "new" => new(&data.packages, &parsed.operands, &answer),
                                    "y" | "symlink" => symlink(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                    ),
                                    "u" | "update" => update(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                        &option,
                                    ),
                                    "b" | "build" => build(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                        &option,
                                    ),
                                    "m" | "move" => relocate(
                                        &data.packages,
                                        &data.bin,
                                        &parsed.operands,
                                        &answer,
                                    ),
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
                                    _ => println!("unknown operator '{}'", operator),
                                };
                            }
                            None => print_usage(color),
                        },
                    },
                }
            }
        }
    }
}
