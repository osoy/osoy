use std::collections::HashMap;

pub fn parse_args<'a>(
    args: &'a [String],
    valid_flags: HashMap<&'a str, &'a str>,
    valid_opts: HashMap<&'a str, &'a str>,
) -> Result<(Vec<String>, Vec<&'a str>, HashMap<&'a str, String>), String> {
    let mut words: Vec<String> = Vec::new();
    let mut flags: Vec<&'a str> = Vec::new();
    let mut opts: HashMap<&str, String> = HashMap::new();
    let mut listener_opt: &str = "";

    for a in args {
        if !listener_opt.is_empty() {
            opts.insert(listener_opt.clone(), a.to_owned());
            listener_opt = "";
        } else {
            if a.starts_with("--") {
                let f = &a[2..];
                if let Some(opt) = valid_opts.get(&f) {
                    listener_opt = *opt;
                } else if valid_flags.contains_key(&f) {
                    flags.push(valid_flags.get(f).unwrap());
                } else {
                    return Err(format!("unknown flag '--{}'", f));
                }
            } else if a.starts_with("-") {
                for c in a.chars().skip(1) {
                    let f = &c.to_string();
                    if let Some(opt) = valid_opts.get(&f.as_str()) {
                        listener_opt = *opt;
                    } else if valid_flags.contains_key(&f.as_str()) {
                        flags.push(valid_flags.get(f.as_str()).unwrap());
                    } else {
                        return Err(format!("unknown flag '-{}'", f));
                    }
                }
            } else {
                words.push(a.to_owned());
            }
        }
    }

    if !listener_opt.is_empty() {
        return Err(format!("option '{}' requires a value", listener_opt));
    }

    Ok((words, flags, opts))
}
