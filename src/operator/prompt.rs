use std::io::{stdin, stdout, Write};

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

pub fn prompt_yes(msg: &str, answer: &Answer) -> bool {
    print!("{} [Y/n] ", msg);
    match answer {
        Answer::Undefined => {
            let _ = stdout().flush();
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
            match answer {
                Answer::Force | Answer::Default => true,
                _ => false,
            }
        }
    }
}

pub fn prompt_no(msg: &str, answer: &Answer) -> bool {
    print!("{} [y/N] ", msg);
    match answer {
        Answer::Undefined => {
            let _ = stdout().flush();
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
            match answer {
                Answer::Force => true,
                _ => false,
            }
        }
    }
}
