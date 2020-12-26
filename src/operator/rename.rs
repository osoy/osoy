use crate::{repos, Config, Exec, Location, StructOpt};
use git2::Repository;

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
            Ok(path) => {
                let dest_path = config.src.join(self.destination.id());
                match repos::rename(&path, &dest_path) {
                    Ok(_) => {
                        if let Err(err) = match Repository::open(&dest_path) {
                            Ok(repo) => repo.remote_set_url("origin", &self.destination.url()),
                            Err(err) => Err(err),
                        } {
                            info!("could not set remote: {}", err);
                        }

                        if self.verbose {
                            info!(
                                "renamed '{}' to '{}'",
                                path.strip_prefix(config.src).unwrap().display(),
                                self.destination.id()
                            );
                        }
                    }
                    Err(err) => info!("rename failed: {}", err),
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
