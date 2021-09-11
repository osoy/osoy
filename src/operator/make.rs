use crate::{repo, Config, Exec, Location};
use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(alias = "mk", about = "Run make in repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, help = "Run interactively")]
    pub interactive: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    pub targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        let interactive = self.interactive;
        let io_dest = || {
            interactive
                .then(|| Stdio::inherit())
                .unwrap_or(Stdio::null())
        };

        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                for path in iter {
                    if interactive {
                        println!("{}", path.strip_prefix(&config.src).unwrap().display());
                    } else {
                        print!("{}..", path.strip_prefix(&config.src).unwrap().display());
                        stdout().flush().ok();
                    }
                    let status = Command::new("make")
                        .current_dir(&path)
                        .env("PWD", path.display().to_string())
                        .stdin(io_dest())
                        .stderr(io_dest())
                        .stdout(io_dest())
                        .status();
                    match status {
                        Ok(status) => {
                            let code = status.code();
                            if code.as_ref().map_or(true, |c| *c != 0) {
                                errors += 1;
                            }
                            println!(
                                "{}",
                                status.code().map_or("NONE".into(), |c| (c != 0)
                                    .then(|| format!("E{}", c))
                                    .unwrap_or("OK".into()))
                            )
                        }
                        Err(err) => {
                            errors += 1;
                            info!("failed to execute make: {}", err)
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
