use crate::gitutil::{clone, FetchMessage};
use crate::{repo, Config, Exec, Location};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone repositories")]
pub struct Opt {
    #[structopt(short, long, default_value = "10", help = "Count of parallel jobs")]
    pub parallel: usize,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
    #[structopt(short, long, help = "Show detailed output")]
    pub verbose: bool,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        let receiver = clone(
            self.targets
                .iter()
                .map(|location| (location.url(), config.src.join(&location.id())))
                .collect(),
            self.parallel,
        );
        while let Ok(msg) = receiver.recv() {
            match msg {
                FetchMessage::Done((path, res, prog)) => {
                    let id = path
                        .strip_prefix(&config.src)
                        .unwrap()
                        .display()
                        .to_string();
                    println!(
                        "{} {}",
                        id,
                        match res {
                            Ok(_) => "done",
                            Err(err) => {
                                if self.verbose {
                                    println!("{}", err);
                                }
                                errors += 1;
                                repo::remove(&config.bin, &path).ok();
                                "failed"
                            }
                        }
                    );
                    prog.print();
                }
                FetchMessage::Progress(prog) => prog.print(),
            }
        }

        errors
    }
}
