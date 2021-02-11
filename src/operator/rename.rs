use crate::{repo, Config, Exec, Location, StructOpt};
use git2::Repository;
use std::io;

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
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;
        let Self {
            regex,
            verbose,
            target,
            destination,
        } = self;

        match repo::unique(&config.src, target.clone(), regex) {
            Ok(path) => {
                let dest_path = config.src.join(destination.id());
                let rename_res = repo::rename(&path, &dest_path);
                if rename_res
                    .as_ref()
                    .err()
                    .map(|err| {
                        err.kind() == io::ErrorKind::AlreadyExists
                            && match regex {
                                true => target.matches_re(&path),
                                false => target.matches(&path),
                            }
                    })
                    .unwrap_or(true)
                {
                    if let Err(err) = Repository::open(&dest_path).map(|repo| {
                        repo.remote_set_url("origin", &destination.url())
                            .and_then(|_| Ok(info!("origin: {}", &destination.url())))
                    }) {
                        errors += 1;
                        info!("could not set remote: {}", err);
                    }
                    if rename_res.is_ok() && verbose {
                        info!(
                            "renamed '{}' to '{}'",
                            path.strip_prefix(config.src).unwrap().display(),
                            destination.id()
                        );
                    }
                } else {
                    errors += 1;
                    info!("rename failed: {}", rename_res.err().unwrap())
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
