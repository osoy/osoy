use crate::gitutil::{clone, FetchMessage};
use crate::{repo, Config, Exec, Location};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone repositories")]
pub struct Opt {
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        let receiver = clone(
            self.targets
                .iter()
                .map(|location| (location.url(), config.src.join(&location.id()))),
        );
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
                            repo::remove(&config.bin, &path).ok();
                            "failed"
                        })
                    );
                }
                FetchMessage::Progress(prog) => prog.print(),
            }
        }

        errors
    }
}
