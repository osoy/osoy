use crate::{gitutil, repo, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let cache = gitutil::AuthCache::new();
        let mut errors = 0;

        for location in self.targets {
            let id = location.id();
            let path = config.src.join(&id);
            if path.exists() {
                info!("entity '{}' already exists", &id)
            } else {
                let res = gitutil::clone(&path, &id, &location.url(), &cache);
                print!("\u{1b}[K");
                match res {
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
