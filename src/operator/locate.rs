use crate::{repo, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(about = "Print repository's full path")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(help = Location::about())]
    target: Location,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        match repo::unique(&config.src, self.target, self.regex) {
            Ok(path) => {
                println!("{}", path.display());
                0
            }
            Err(err) => {
                info!("{}", err);
                1
            }
        }
    }
}
