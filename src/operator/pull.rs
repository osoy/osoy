use crate::gitutil::{pull, FetchMessage};
use crate::{repo, Config, Exec, Location};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Pull from repository remotes")]
pub struct Opt {
    #[structopt(short, long, default_value = "10", help = "Count of parallel jobs")]
    pub parallel: usize,
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                let receiver = pull(iter.collect(), self.parallel);
                while let Ok(msg) = receiver.recv() {
                    match msg {
                        FetchMessage::Done((path, res)) => {
                            let id = path
                                .strip_prefix(&config.src)
                                .unwrap()
                                .display()
                                .to_string();
                            println!(
                                "{} {}",
                                id,
                                res.is_ok().then(|| "done").unwrap_or_else(|| {
                                    errors += 1;
                                    "failed"
                                })
                            );
                        }
                        FetchMessage::Progress(prog) => prog.print(),
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
