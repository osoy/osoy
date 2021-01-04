use crate::{link, repo, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(alias = "ls", about = "List repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(short, long, help = "List executables", parse(from_occurrences))]
    executables: u8,
    #[structopt(help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let exe_flag = self.executables;

                let symlinks = match exe_flag {
                    0 => None,
                    _ => Some(
                        link::entries(&config.bin)
                            .map(|iter| iter.collect())
                            .unwrap_or(vec![]),
                    ),
                };

                for path in iter {
                    let exes = match exe_flag {
                        0 => None,
                        _ => link::executables(&path).ok().map(|iter| {
                            iter.filter_map(|exe| {
                                let symbolics = symlinks
                                    .clone()
                                    .unwrap()
                                    .iter()
                                    .filter_map(|(sym, dest)| match dest == &exe {
                                        true => {
                                            Some(sym.strip_prefix(&config.bin).unwrap().display())
                                        }
                                        false => None,
                                    })
                                    .fold(String::new(), |acc, sym| match acc.is_empty() {
                                        true => format!(" <- {}", sym),
                                        false => format!("{}, {}", acc, sym),
                                    });

                                match exe_flag == 1 || !symbolics.is_empty() {
                                    true => Some(format!(
                                        "{}{}",
                                        exe.strip_prefix(&path).unwrap().display(),
                                        symbolics,
                                    )),
                                    false => None,
                                }
                            })
                            .collect::<Vec<String>>()
                        }),
                    };

                    println!(
                        "{}{}",
                        path.strip_prefix(&config.src).unwrap().display(),
                        exes.map(|lines| lines
                            .iter()
                            .fold(String::new(), |acc, line| format!("{}\n  {}", acc, line)))
                            .unwrap_or(String::new())
                    );
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
