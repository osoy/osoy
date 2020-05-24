use std::collections::HashMap;

pub fn parse_args(
    args: &[String],
    valid_flags: &[&str],
    valid_opts: &[&str],
) -> Result<(Vec<String>, Vec<String>, HashMap<String, String>), String> {
    let mut words: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    let mut opts: HashMap<String, String> = HashMap::new();
    let mut listener_opt = String::new();

    for a in args {
        if !listener_opt.is_empty() {
            opts.insert(listener_opt.clone(), String::from(a));
            listener_opt.clear();
        } else {
            if a.starts_with("--") {
                let f = &a[2..];
                if valid_opts.contains(&f) {
                    listener_opt = String::from(f);
                } else if valid_flags.contains(&f) {
                    flags.push(String::from(f));
                } else {
                    return Err(format!("unknown flag '--{}'", f));
                }
            } else if a.starts_with("-") {
                for c in a.chars().skip(1) {
                    let f = &c.to_string();
                    if valid_opts.contains(&f.as_str()) {
                        listener_opt = String::from(f);
                    } else if valid_flags.contains(&f.as_str()) {
                        flags.push(String::from(f));
                    } else {
                        return Err(format!("unknown flag '-{}'", f));
                    }
                }
            } else {
                words.push(String::from(a));
            }
        }
    }

    if !listener_opt.is_empty() {
        return Err(format!("option '-{}' requires a value", listener_opt));
    }

    Ok((words, flags, opts))
}
