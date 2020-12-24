use crate::{repos, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug, Clone)]
#[structopt(alias = "ls", about = "List repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repos::iter_repos_matching(&config.src, self.targets, self.regex) {
            Ok(iter) => iter.for_each(|path| {
                path.strip_prefix(&config.src)
                    .ok()
                    .map(|rel| println!("{}", rel.display()));
            }),
            Err(err) => info!("could not access '{}': {}", config.src.display(), err),
        }
    }
}
