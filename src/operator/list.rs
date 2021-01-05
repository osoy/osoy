use crate::{link, repo, Config, Exec, Location, StructOpt};
use git2::{Repository, Status};
use structopt::clap::ArgGroup;

#[derive(StructOpt, Debug)]
#[structopt(alias = "ls", about = "List repositories", group = ArgGroup::with_name("sublist"))]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(
        short,
        long,
        group = "sublist",
        help = "List executables",
        parse(from_occurrences)
    )]
    exe: u8,
    #[structopt(
        short,
        long,
        group = "sublist",
        help = "Show git statuses",
        parse(from_occurrences)
    )]
    git: u8,
    #[structopt(short, long, help = "Show only entries with details")]
    only_details: bool,
    #[structopt(help = Location::about())]
    targets: Vec<Location>,
}

fn status_wt_char(st: &Status) -> Option<char> {
    match st {
        s if s.contains(Status::WT_NEW) => Some('N'),
        s if s.contains(Status::WT_MODIFIED) => Some('M'),
        s if s.contains(Status::WT_DELETED) => Some('D'),
        s if s.contains(Status::WT_RENAMED) => Some('R'),
        s if s.contains(Status::WT_TYPECHANGE) => Some('T'),
        _ => None,
    }
}

fn status_index_char(st: &Status) -> Option<char> {
    match st {
        s if s.contains(Status::INDEX_NEW) => Some('N'),
        s if s.contains(Status::INDEX_MODIFIED) => Some('M'),
        s if s.contains(Status::INDEX_DELETED) => Some('D'),
        s if s.contains(Status::INDEX_RENAMED) => Some('R'),
        s if s.contains(Status::INDEX_TYPECHANGE) => Some('T'),
        _ => None,
    }
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let exe_flag = self.exe;
                let git_flag = self.git;

                let symlinks = match exe_flag {
                    0 => None,
                    _ => Some(
                        link::entries(&config.bin)
                            .map(|iter| iter.collect())
                            .unwrap_or(vec![]),
                    ),
                };

                for path in iter {
                    let exe_lines = match exe_flag {
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
                            .fold(String::new(), |acc, line| format!("{}\n  {}", acc, line))
                        }),
                    }
                    .unwrap_or("".into());

                    let git_lines = match git_flag {
                        0 => None,
                        _ => Repository::open(&path)
                            .ok()
                            .map(|repo| {
                                repo.statuses(None).ok().map(|statuses| {
                                    statuses
                                        .iter()
                                        .map(|entry| {
                                            let st = entry.status();
                                            let index_ch = status_index_char(&st);
                                            let wt_ch = match git_flag {
                                                1 => status_wt_char(&st),
                                                _ => None,
                                            };
                                            let fname = String::from_utf8_lossy(entry.path_bytes());
                                            vec![
                                                index_ch
                                                    .map(|ch| format!("{}* {}", ch, fname))
                                                    .unwrap_or("".into()),
                                                wt_ch
                                                    .map(|ch| format!("{}  {}", ch, fname))
                                                    .unwrap_or("".into()),
                                            ]
                                        })
                                        .flatten()
                                        .filter(|word| !word.is_empty())
                                        .fold(String::new(), |acc, line| {
                                            format!("{}\n  {}", acc, line)
                                        })
                                })
                            })
                            .flatten(),
                    }
                    .unwrap_or("".into());

                    if !self.only_details || !exe_lines.is_empty() || !git_lines.is_empty() {
                        println!(
                            "{}{}{}",
                            path.strip_prefix(&config.src).unwrap().display(),
                            exe_lines,
                            git_lines
                        );
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
