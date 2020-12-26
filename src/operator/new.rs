use crate::{Config, Exec, Location, StructOpt};
use git2::Repository;

#[derive(StructOpt, Debug)]
#[structopt(alias = "n", about = "Create new empty git repositories")]
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
                match Repository::init(path) {
                    Ok(repo) => {
                        if self.verbose {
                            info!("new repository created '{}'", location.id());
                        }
                        if let Err(err) = repo.remote("origin", &location.url()) {
                            info!("could not set remote: {}", err);
                        }
                    }
                    Err(err) => info!("could not init: {}", err),
                }
            }
        }
    }
}
