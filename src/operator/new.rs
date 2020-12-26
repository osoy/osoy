use crate::{Config, Exec, Location, StructOpt};
use git2::Repository;

#[derive(StructOpt, Debug)]
#[structopt(alias = "n", about = "Create new empty git repositories")]
pub struct Opt {
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        for location in self.targets {
            let path = config.src.join(location.id());
            match path.exists() {
                true => info!("entity '{}' already exists", path.display()),
                false => match Repository::init(path) {
                    Ok(repo) => {
                        if let Err(err) = repo.remote("origin", &location.url()) {
                            info!("could not set remote: {}", err);
                        }
                    }
                    Err(err) => info!("could not init: {}", err),
                },
            }
        }
    }
}
