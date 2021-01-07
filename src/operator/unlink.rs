use crate::{link, repo, Config, Exec, Location, StructOpt};
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(about = "Remove symbolic links")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(short, long, help = "Do not prompt")]
    force: bool,
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                if let Ok(iter) = link::iterate(&config.bin, iter.collect()) {
                    for (sym, dest) in iter {
                        if self.force
                            || ask_bool!(
                                "unlink '{}'?",
                                dest.strip_prefix(&config.src).unwrap().display()
                            )
                        {
                            match fs::remove_file(&sym) {
                                Ok(_) => {
                                    if self.verbose {
                                        info!(
                                            "removed '{}'",
                                            sym.strip_prefix(&config.bin).unwrap().display()
                                        );
                                    }
                                }
                                Err(err) => {
                                    errors += 1;
                                    info!("could not remove '{}': {}", sym.display(), err)
                                }
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
