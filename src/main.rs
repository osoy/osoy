use dirs::home_dir;
use std::env::args;

mod output;
use output::{error, msg, print_usage};

mod args;
use args::parse_args;

mod operator;
use operator::list;

fn main() {
    match parse_args(&args().collect::<Vec<String>>()[1..], &["c"], &["d"]) {
        Err(msg) => error(&msg),
        Ok((flags, words, opts)) => {
            println!("words:{}", words.join(","),);
            println!("flags:{}", flags.join(","),);
            println!(
                "opts:{}",
                opts.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join(",")
            );

            if let Some(home) = home_dir() {
                let osoy_path = home.join(".osoy");
                if osoy_path.is_dir() {
                    match words.get(0) {
                        Some(operator) => match operator.as_str() {
                            "l" | "list" => list(
                                osoy_path.join("packages").as_path(),
                                osoy_path.join("bin").as_path(),
                                &words[1..],
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
    }
}
