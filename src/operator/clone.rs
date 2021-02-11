use crate::{gitutil, repo, Config, Exec, Location};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Print what is being done")]
    pub verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let auth_cache = gitutil::AuthCache::new();
        let mut errors = 0;

        for location in self.targets {
            let id = location.id();
            let path = config.src.join(&id);

            if path.exists() {
                info!("entity '{}' already exists", &id)
            } else {
                match gitutil::clone(&path, &id, &location.url(), &auth_cache) {
                    Ok(_) => gitutil::log("done", id),
                    Err(err) => {
                        errors += 1;
                        gitutil::log("failed", id);
                        if self.verbose {
                            gitutil::log("", err);
                        }
                        repo::remove(&config.bin, &path).ok();
                    }
                }
            }
        }

        errors
    }
}
