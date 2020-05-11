use std::env;
mod usage;

fn clone(args: Vec<String>) {
    for a in args.iter() {}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(operator) => match operator.as_str() {
            "c" | "clone" => {
                clone(args[2..].to_vec());
            }
            "r" | "remove" => usage::msg("to be implemented: remove <query>"),
            "s" | "symlink" => usage::msg("to be implemented: symlink [query]"),
            "u" | "update" => usage::msg("to be implemented: update [query]"),
            "l" | "list" => usage::msg("to be implemented: list [query]"),
            "m" | "make" => usage::msg("to be implemented: make [query]"),
            "dir" => usage::msg("to be implemented: dir <query>"),
            "read" => usage::msg("to be implemented: read <query>"),
            "license" => usage::msg("to be implemented: license <query>"),
            _ => usage::error(&format!("unknown operator '{}'", operator)),
        },
        None => usage::print_usage(),
    }
}
