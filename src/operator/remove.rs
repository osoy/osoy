use crate::{repo, Config, Exec, Location};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(alias = "rm", about = "Remove repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, help = "Do not prompt")]
    pub force: bool,
    #[structopt(short, long, help = "Print what is being done")]
    pub verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                for path in iter {
                    let path_display = path.strip_prefix(&config.src).unwrap().display();
                    if self.force || ask_bool!("remove '{}'?", path_display) {
                        match repo::remove(&config.bin, &path) {
                            Ok(_) => {
                                if self.verbose {
                                    info!("removed '{}'", path_display);
                                }
                            }
                            Err(err) => {
                                errors += 1;
                                info!("could not remove '{}': {}", path.display(), err)
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
