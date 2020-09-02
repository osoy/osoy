use std::collections::HashMap;

#[derive(Default)]
pub struct ParsedArgs<'a> {
    pub words: Vec<String>,
    pub flags: Vec<&'a str>,
    pub options: HashMap<&'a str, String>,
}

pub fn parse_args<'a>(
    args: &'a [String],
    valid_flags: &'a [&'a [&'a str]],
    valid_opts: &'a [&'a [&'a str]],
) -> Result<ParsedArgs<'a>, String> {
    let mut mut_flags: HashMap<&str, &str> = HashMap::new();
    let mut mut_opts: HashMap<&str, &str> = HashMap::new();
    for fs in valid_flags.iter() {
        let key = fs[0];
        for f in fs.iter() {
            mut_flags.insert(f, key);
        }
    }
    for fs in valid_opts.iter() {
        let key = fs[0];
        for f in fs.iter() {
            mut_opts.insert(f, key);
        }
    }
    let valid_flags = mut_flags;
    let valid_opts = mut_opts;

    let mut parsed_args = ParsedArgs::default();
    let mut listener_opt: &str = "";

    for a in args {
        if !listener_opt.is_empty() {
            parsed_args
                .options
                .insert(listener_opt.clone(), a.to_owned());
            listener_opt = "";
        } else {
            if a.starts_with("--") {
                let f = &a[2..];
                if let Some(opt) = valid_opts.get(&f) {
                    listener_opt = *opt;
                } else if valid_flags.contains_key(&f) {
                    parsed_args.flags.push(valid_flags.get(f).unwrap());
                } else {
                    return Err(format!("unknown flag '--{}'", f));
                }
            } else if a.starts_with("-") {
                for c in a.chars().skip(1) {
                    let f = &c.to_string();
                    if let Some(opt) = valid_opts.get(&f.as_str()) {
                        listener_opt = *opt;
                    } else if valid_flags.contains_key(&f.as_str()) {
                        parsed_args.flags.push(valid_flags.get(f.as_str()).unwrap());
                    } else {
                        return Err(format!("unknown flag '-{}'", f));
                    }
                }
            } else {
                parsed_args.words.push(a.to_owned());
            }
        }
    }

    if !listener_opt.is_empty() {
        return Err(format!("option '{}' requires a value", listener_opt));
    }

    Ok(parsed_args)
}
