use crate::{repos, transfer, Config, Exec, Location, StructOpt};
use std::sync::{Arc, Mutex};

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        let cache = Arc::new(Mutex::new(transfer::cache()));

        for location in self.targets {
            let id = location.id();
            let path = config.src.join(&id);
            if path.exists() {
                info!("entity '{}' already exists", &id)
            } else {
                let res = transfer::clone(&path, &id, &location.url(), &cache);
                print!("\u{1b}[K");
                match res {
                    Ok(_) => transfer::log("done", id),
                    Err(err) => {
                        transfer::log("failed", id);
                        if self.verbose {
                            transfer::log("", err);
                        }
                        repos::remove(&path).ok();
                    }
                }
            }
        }
    }
}
