use crate::{repos, Config, Exec, Location, StructOpt};
use std::io;

#[derive(StructOpt, Debug)]
#[structopt(alias = "ls", about = "List repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repos::iter_repos_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => iter.for_each(|path| {
                path.strip_prefix(&config.src)
                    .ok()
                    .map(|rel| println!("{}", rel.display()));
            }),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => info!("no repositories found"),
                _ => info!("{}", err),
            },
        }
    }
}
