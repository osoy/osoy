use crate::{repo, Config, Exec, Location};
use std::env::set_current_dir;
use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(aliases = &["ex", "exec"], about = "Execute command in a repository")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, help = "Less output")]
    pub quiet: bool,
    #[structopt(help = Location::about())]
    pub target: Location,
    #[structopt(help = "Command to execute in the repository")]
    pub command: String,
    #[structopt(help = "Arguments for the command")]
    pub arguments: Vec<String>,
}

impl Exec for Opt {
    fn exec(self, config: Config) -> i32 {
        let mut errors = 0;

        let quiet = self.quiet;
        let io_dest = || quiet.then(|| Stdio::null()).unwrap_or(Stdio::inherit());

        match repo::iterate_matching_exists(&config.src, vec![self.target], self.regex) {
            Ok(iter) => {
                for path in iter {
                    if quiet {
                        print!("{}..", path.strip_prefix(&config.src).unwrap().display());
                        stdout().flush().ok();
                    } else {
                        println!("{}", path.strip_prefix(&config.src).unwrap().display());
                    }
                    match set_current_dir(&path) {
                        Ok(_) => match Command::new(&self.command)
                            .args(&self.arguments)
                            .stdin(io_dest())
                            .stderr(io_dest())
                            .stdout(io_dest())
                            .env("PWD", path.display().to_string())
                            .status()
                        {
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
                                info!("failed to execute '{}': {}", self.command, err)
                            }
                        },
                        Err(err) => {
                            errors += 1;
                            info!("could not access '{}': {}", path.display(), err)
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
