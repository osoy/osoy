use crate::{link, repo, Config, Exec, Location, StructOpt};
use git2::{Repository, Status};
use structopt::clap::ArgGroup;

#[derive(StructOpt, Debug)]
#[structopt(alias = "ls", about = "List repositories", group = ArgGroup::with_name("sublist"))]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(short, long, group = "sublist", help = "List executables")]
    exe: bool,
    #[structopt(
        short = "E",
        long,
        group = "sublist",
        help = "List executables that are linked"
    )]
    exe_linked: bool,
    #[structopt(short, long, group = "sublist", help = "Show git statuses")]
    git: bool,
    #[structopt(short, long, help = "Show only entries with details")]
    only_details: bool,
    #[structopt(help = Location::about())]
    targets: Vec<Location>,
}

fn status_wt_char(st: &Status) -> Option<&'static char> {
    match st {
        s if s.contains(Status::WT_NEW) => Some(&'N'),
        s if s.contains(Status::WT_MODIFIED) => Some(&'M'),
        s if s.contains(Status::WT_DELETED) => Some(&'D'),
        s if s.contains(Status::WT_RENAMED) => Some(&'R'),
        s if s.contains(Status::WT_TYPECHANGE) => Some(&'T'),
        _ => None,
    }
}

fn status_index_char(st: &Status) -> Option<&'static char> {
    match st {
        s if s.contains(Status::INDEX_NEW) => Some(&'N'),
        s if s.contains(Status::INDEX_MODIFIED) => Some(&'M'),
        s if s.contains(Status::INDEX_DELETED) => Some(&'D'),
        s if s.contains(Status::INDEX_RENAMED) => Some(&'R'),
        s if s.contains(Status::INDEX_TYPECHANGE) => Some(&'T'),
        _ => None,
    }
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let flag_exe_linked = self.exe_linked;
                let flag_exe = self.exe || flag_exe_linked;

                let symlinks = match flag_exe {
                    false => None,
                    true => Some(
                        link::entries(&config.bin)
                            .map(|iter| iter.collect())
                            .unwrap_or(vec![]),
                    ),
                };

                for path in iter {
                    let lines_exe = match flag_exe {
                        false => None,
                        true => link::executables(&path).ok().map(|iter| {
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

                                match !flag_exe_linked || !symbolics.is_empty() {
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
                    };

                    let (lines_git, branch) = match self.git {
                        false => None,
                        true => Repository::open(&path).ok().map(|repo| {
                            (
                                repo.statuses(None).ok().map(|statuses| {
                                    statuses
                                        .iter()
                                        .map(|entry| {
                                            let st = entry.status();
                                            let index_ch = status_index_char(&st);
                                            let wt_ch = status_wt_char(&st);
                                            let fname = String::from_utf8_lossy(entry.path_bytes());
                                            vec![
                                                index_ch
                                                    .map(|ch| {
                                                        format!("{} {}", ch.to_uppercase(), fname)
                                                    })
                                                    .unwrap_or("".into()),
                                                wt_ch
                                                    .map(|ch| {
                                                        format!("{} {}", ch.to_lowercase(), fname)
                                                    })
                                                    .unwrap_or("".into()),
                                            ]
                                        })
                                        .flatten()
                                        .filter(|word| !word.is_empty())
                                        .fold(String::new(), |acc, line| {
                                            format!("{}\n  {}", acc, line)
                                        })
                                }),
                                repo.head().ok().map(|head| {
                                    String::from_utf8_lossy(head.shorthand_bytes()).to_string()
                                }),
                            )
                        }),
                    }
                    .unwrap_or((None, None));

                    if !self.only_details
                        || lines_exe.as_ref().map(|l| !l.is_empty()).unwrap_or(false)
                        || lines_git.as_ref().map(|l| !l.is_empty()).unwrap_or(false)
                        || branch.as_ref().map(|b| b != "master").unwrap_or(false)
                    {
                        println!(
                            "{}{}{}{}",
                            path.strip_prefix(&config.src).unwrap().display(),
                            branch.map(|b| format!(":{}", b)).unwrap_or("".into()),
                            lines_exe.unwrap_or("".into()),
                            lines_git.unwrap_or("".into())
                        );
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
