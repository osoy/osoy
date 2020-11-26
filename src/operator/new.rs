use crate::{Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug, Clone)]
#[structopt(alias = "n", about = "Create new empty git repositories")]
pub struct New {
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for New {
    fn exec(self, config: Config) {
        for location in self.targets {
            let path = &config.src.join(location.id());
            match path.exists() {
                true => info!("entity '{}' already exists", path.display()),
                false => match git2::Repository::init(&path) {
                    Ok(repo) => match repo.remote("origin", &location.url()) {
                        Ok(_) => {}
                        Err(err) => info!("could not set remote: {}", err),
                    },
                    Err(err) => info!("could not init: {}", err),
                },
            }
        }
    }
}
