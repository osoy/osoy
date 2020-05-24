use std::io::{stdin, stdout, Write};

pub fn prompt_yes(msg: &str) -> bool {
    print!("{} [Y/n] ", msg);
    let _ = stdout().flush();
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "Y" | "y" | "" => return true,
            _ => return false,
        },
        Err(_) => {}
    }
    false
}

pub fn prompt_no(msg: &str) -> bool {
    print!("{} [y/N] ", msg);
    let _ = stdout().flush();
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "Y" | "y" => return true,
            _ => return false,
        },
        Err(_) => {}
    }
    false
}
