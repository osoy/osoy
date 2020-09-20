use std::io::{stdin, stdout, Write};

#[derive(PartialEq)]
pub enum Answer {
    Force,
    Default,
    Deny,
    Undefined,
}

impl Answer {
    pub fn new(force: bool, default: bool, deny: bool) -> Result<Answer, String> {
        if !(force || default || deny) {
            return Ok(Answer::Undefined);
        } else if force && !(default || deny) {
            return Ok(Answer::Force);
        } else if default && !(force || deny) {
            return Ok(Answer::Default);
        } else if deny && !(force || default) {
            return Ok(Answer::Deny);
        } else {
            let mut list = Vec::new();
            if force {
                list.push("force");
            }
            if default {
                list.push("default");
            }
            if deny {
                list.push("deny");
            }
            return Err(format!("answer conflict: {}", list.join(", ")));
        }
    }
}

pub fn prompt(msg: &str, answer: &Answer) -> bool {
    print!("{} [y/n] ", msg);
    match answer {
        Answer::Undefined | Answer::Default => loop {
            stdout().flush().unwrap();
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => match input.trim() {
                    "Y" | "y" => return true,
                    "N" | "n" => return false,
                    _ => print!("{} [y/n] ", msg),
                },
                Err(msg) => println!("error: {}", msg),
            }
        },
        _ => {
            println!();
            return answer == &Answer::Force;
        }
    }
}

pub fn prompt_yes(msg: &str, answer: &Answer) -> bool {
    print!("{} [Y/n] ", msg);
    match answer {
        Answer::Undefined => {
            stdout().flush().unwrap();
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => match input.trim() {
                    "Y" | "y" | "" => return true,
                    _ => return false,
                },
                Err(msg) => println!("error: {}", msg),
            }
            true
        }
        _ => {
            println!();
            return answer == &Answer::Force || answer == &Answer::Default;
        }
    }
}

pub fn prompt_no(msg: &str, answer: &Answer) -> bool {
    print!("{} [y/N] ", msg);
    match answer {
        Answer::Undefined => {
            stdout().flush().unwrap();
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => match input.trim() {
                    "Y" | "y" => return true,
                    _ => return false,
                },
                Err(msg) => println!("error: {}", msg),
            }
            false
        }
        _ => {
            println!();
            return answer == &Answer::Force;
        }
    }
}
