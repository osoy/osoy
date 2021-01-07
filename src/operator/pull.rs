use crate::{gitutil, repo, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(about = "Pull from repository remotes")]
pub struct Opt {
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let cache = gitutil::AuthCache::new();

                for path in iter {
                    let id = path
                        .strip_prefix(&config.src)
                        .unwrap()
                        .display()
                        .to_string();

                    match gitutil::pull(&path, &id, &cache) {
                        Ok(_) => gitutil::log("done", id),
                        Err(err) => {
                            errors += 1;
                            gitutil::log("failed", id);
                            if self.verbose && !format!("{}", err).is_empty() {
                                gitutil::log("", err);
                            }
                        }
                    }
                }
            }
            Err(err) => {
                errors += 1;
                info!("{}", err)
            }
        }

        errors
    }
}
