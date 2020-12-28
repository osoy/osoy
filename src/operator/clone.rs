use crate::{auth, Config, Exec, Location, StructOpt};
use git2::build::RepoBuilder;
use git2::{FetchOptions, RemoteCallbacks};

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
            let id = location.id();
            let path = config.src.join(&id);
            if path.exists() {
                info!("entity '{}' already exists", &id)
            } else {
                let mut callbacks = RemoteCallbacks::new();

                {
                    let id = id.clone();
                    callbacks.credentials(move |_, username, allowed_types| {
                        auth::credentials(&id, username, allowed_types)
                    });
                }

                let mut options = FetchOptions::new();
                options.remote_callbacks(callbacks);

                let mut builder = RepoBuilder::new();
                builder.fetch_options(options);

                match builder.clone(&location.url(), &path) {
                    Ok(_) => {
                        if self.verbose {
                            info!("repository cloned '{}'", id);
                        }
                    }
                    Err(err) => info!("could not clone: {}", err),
                }
            }
        }
    }
}
