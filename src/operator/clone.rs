use crate::{Config, Exec, Location, StructOpt};
use git2::Repository;

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone from remote repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        for location in self.targets {
            let path = config.src.join(location.id());
            if path.exists() {
                info!("entity '{}' already exists", location.id())
            } else {
                match Repository::clone(&location.url(), path) {
                    Ok(_) => {
                        if self.verbose {
                            info!("repository cloned '{}'", location.id());
                        }
                    }
                    Err(err) => info!("could not clone: {}", err),
                }
            }
        }
    }
}
