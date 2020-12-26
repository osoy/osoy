use crate::{repos, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(aliases = &["mv", "move"], about = "Relocate repository")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(help = Location::about())]
    target: Location,
    #[structopt(help = Location::about())]
    destination: Location,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repos::unique(&config.src, self.target, self.regex) {
            Ok(repo) => match repos::rename(&repo, &config.src.join(self.destination.id())) {
                Ok(parents) => {
                    if self.verbose {
                        info!(
                            "renamed '{}' to '{}'{}",
                            repo.strip_prefix(config.src).unwrap().display(),
                            self.destination.id(),
                            match parents {
                                0 => "".into(),
                                _ => format!(
                                    " and removed {} empty director{}",
                                    parents,
                                    match parents {
                                        1 => "y",
                                        _ => "ies",
                                    }
                                ),
                            }
                        );
                    }
                }
                Err(err) => info!("rename failed: {}", err),
            },
            Err(err) => info!("{}", err),
        }
    }
}
