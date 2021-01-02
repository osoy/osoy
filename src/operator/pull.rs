use crate::{repos, transfer, Config, Exec, Location, StructOpt};
use std::sync::{Arc, Mutex};

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
    fn exec(self, config: Config) {
        match repos::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let cache = Arc::new(Mutex::new(transfer::cache()));

                for path in iter {
                    let id = path
                        .strip_prefix(&config.src)
                        .unwrap()
                        .display()
                        .to_string();

                    match transfer::pull(&path, &id, &cache) {
                        Ok(_) => transfer::log("done", id),
                        Err(err) => {
                            transfer::log("failed", id);
                            if self.verbose && !format!("{}", err).is_empty() {
                                transfer::log("", err);
                            }
                        }
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
