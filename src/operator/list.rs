use crate::{gitutil, link, repo, Config, Exec, Location, StructOpt};
use git2::Repository;
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

                    let git_status = match self.git {
                        false => None,
                        true => Repository::open(&path)
                            .ok()
                            .map(|repo| gitutil::RepoStatus::from(&repo)),
                    };

                    let (lines_git, branch, graph) = git_status
                        .map(|stat| {
                            (
                                stat.changes.map(|changes| {
                                    changes.iter().fold(String::new(), |acc, (ch, fname)| {
                                        format!("{}\n  {} {}", acc, ch, fname)
                                    })
                                }),
                                stat.branch,
                                stat.graph,
                            )
                        })
                        .unwrap_or((None, None, None));

                    if !self.only_details
                        || lines_exe.as_ref().map(|l| !l.is_empty()).unwrap_or(false)
                        || lines_git.as_ref().map(|l| !l.is_empty()).unwrap_or(false)
                        || branch.as_ref().map(|b| b != "master").unwrap_or(self.git)
                        || graph
                            .as_ref()
                            .map(|(ahead, behind)| *ahead != 0 || *behind != 0)
                            .unwrap_or(self.git)
                    {
                        println!(
                            "{}",
                            [
                                path.strip_prefix(&config.src)
                                    .unwrap()
                                    .display()
                                    .to_string(),
                                branch.map(|b| format!(":{}", b)).unwrap_or("".into()),
                                graph
                                    .map(|g| format!(" [{}:{}]", g.0, g.1))
                                    .unwrap_or("".into()),
                                lines_exe.unwrap_or("".into()),
                                lines_git.unwrap_or("".into()),
                            ]
                            .join("")
                        );
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
