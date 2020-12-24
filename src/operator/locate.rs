use crate::{repos, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "Print repository's full path")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(help = Location::about())]
    pub target: Location,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repos::unique_repo(&config.src, self.target, self.regex) {
            Ok(path) => println!("{}", path.display()),
            Err(err) => info!("{}", err),
        }
    }
}
