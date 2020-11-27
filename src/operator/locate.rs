use crate::{util, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "Print repository's full path")]
pub struct Locate {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(help = Location::about())]
    pub target: Location,
}

impl Exec for Locate {
    fn exec(self, config: Config) {
        match util::unique_repo(&config.src, self.target, self.regex) {
            Ok(path) => println!("{}", path.display()),
            Err(err) => info!("{}", err),
        }
    }
}
