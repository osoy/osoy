use crate::{transfer, Config, Exec, Location, StructOpt};
use git2::build::RepoBuilder;
use git2::{FetchOptions, RemoteCallbacks};
use std::sync::{Arc, Mutex};

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
        let cache = Arc::new(Mutex::new(transfer::cache()));

        for location in self.targets {
            let id = location.id();
            let path = config.src.join(&id);
            if path.exists() {
                info!("entity '{}' already exists", &id)
            } else {
                let mut callbacks = RemoteCallbacks::new();

                {
                    let id = id.clone();
                    let cache = cache.clone();
                    callbacks.credentials(move |_, username, allowed_types| {
                        cache
                            .lock()
                            .unwrap()
                            .credentials(&id, username, allowed_types)
                    });
                }

                {
                    let id = id.clone();
                    callbacks.transfer_progress(move |stat| transfer::log_progress(&id, &stat));
                }

                let mut options = FetchOptions::new();
                options.remote_callbacks(callbacks);

                let res = RepoBuilder::new()
                    .fetch_options(options)
                    .clone(&location.url(), &path);

                print!("\u{1b}[K");

                match res {
                    Ok(_) => println!("{:>9} {}", "done", id),
                    Err(err) => println!(
                        "{:>9} {}{}",
                        "failed",
                        id,
                        match self.verbose {
                            false => "".into(),
                            true => format!("\n{:10}{}", "", err),
                        }
                    ),
                }
            }
        }
    }
}
