use crate::{repos, Config, Exec, Location, StructOpt};
use std::fs;

#[derive(StructOpt, Debug, Clone)]
#[structopt(alias = "rm", about = "Remove repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, help = "Do not prompt")]
    pub force: bool,
    #[structopt(short, long, help = "Print what is being done")]
    pub verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repos::iter_repos_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                for path in iter {
                    let path_display = path.strip_prefix(&config.src).unwrap().display();
                    if self.force || ask!("remove '{}'?", path_display) {
                        match fs::remove_dir_all(&path) {
                            Ok(_) => {
                                if self.verbose {
                                    info!("removed '{}'", path_display)
                                }
                            }
                            Err(err) => info!("could not remove '{}': {}", path.display(), err),
                        }
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
