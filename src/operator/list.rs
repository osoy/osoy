use crate::{gitutil, link, repo, Config, Exec, Location};
use git2::Repository;
use std::path::{Path, PathBuf};
use structopt::clap::ArgGroup;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(alias = "ls", about = "List repositories", group = ArgGroup::with_name("sublist"))]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, group = "sublist", help = "List executables")]
    pub exe: bool,
    #[structopt(
        short = "E",
        long,
        group = "sublist",
        help = "List executables that are linked"
    )]
    pub exe_linked: bool,
    #[structopt(short, long, group = "sublist", help = "Show git statuses")]
    pub git: bool,
    #[structopt(short, long, help = "Show only entries with details")]
    pub only_details: bool,
    #[structopt(help = Location::about())]
    pub targets: Vec<Location>,
}

fn executable_listing(
    repo_path: &Path,
    bin_path: &Path,
    symlinks: &[(PathBuf, PathBuf)],
) -> Vec<String> {
    link::executables(repo_path).map_or(vec![], |iter| {
        iter.filter_map(|exe| {
            let symbolics = symlinks
                .clone()
                .iter()
                .filter_map(|(sym, dest)| {
                    (dest == &exe).then(|| sym.strip_prefix(bin_path).unwrap().display())
                })
                .fold(String::new(), |state, sym| match state.is_empty() {
                    true => format!(" <- {}", sym),
                    false => format!("{}, {}", state, sym),
                });

            (!symbolics.is_empty()).then(|| {
                format!(
                    "{}{}",
                    exe.strip_prefix(&repo_path).unwrap().display(),
                    symbolics,
                )
            })
        })
        .collect()
    })
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let flag_exe_linked = self.exe_linked;
                let flag_exe = self.exe || flag_exe_linked;

                let symlinks = flag_exe
                    .then(|| link::entries(&config.bin).map_or(vec![], |iter| iter.collect()));

                for path in iter {
                    let exe_listing = symlinks.as_ref().map_or(String::new(), |symlinks| {
                        executable_listing(&path, &config.bin, symlinks).join("\n  ")
                    });

                    let git_status = self
                        .git
                        .then(|| {
                            Repository::open(&path)
                                .ok()
                                .map(|repo| gitutil::RepoStatus::from(&repo))
                        })
                        .flatten();

                    let (git_listing, branch, graph) =
                        git_status.map_or((String::new(), None, None), |stat| {
                            (
                                stat.changes.map_or(String::new(), |changes| {
                                    changes.iter().fold(String::new(), |state, (ch, fname)| {
                                        format!("{}\n  {} {}", state, ch, fname)
                                    })
                                }),
                                stat.branch,
                                stat.graph,
                            )
                        });

                    if !self.only_details
                        || !exe_listing.is_empty()
                        || !git_listing.is_empty()
                        || graph.as_ref().map_or(false, |g| g.0 * g.1 != 0)
                    {
                        println!(
                            "{}",
                            [
                                path.strip_prefix(&config.src)
                                    .unwrap()
                                    .display()
                                    .to_string(),
                                branch.map_or(String::new(), |b| format!(":{}", b)),
                                graph.map_or(String::new(), |g| format!(" [{}:{}]", g.0, g.1)),
                                exe_listing,
                                git_listing,
                            ]
                            .join("")
                        );
                    }
                }
            }
            Err(err) => {
                errors += 1;
                info!("{}", err)
            }
        }

        errors
    }
}
