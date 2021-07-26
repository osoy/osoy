use crate::{repo, Config, Exec, Location};
use std::env::set_current_dir;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(aliases = &["ex", "exec"], about = "Execute command in a repository")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, help = "Show detailed output")]
    pub verbose: bool,
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

        match repo::iterate_matching_exists(&config.src, vec![self.target], self.regex) {
            Ok(iter) => {
                for path in iter {
                    match set_current_dir(&path) {
                        Ok(_) => match process::Command::new(&self.command)
                            .args(&self.arguments)
                            .stdin(match self.verbose {
                                true => process::Stdio::inherit(),
                                false => process::Stdio::null(),
                            })
                            .stderr(match self.verbose {
                                true => process::Stdio::inherit(),
                                false => process::Stdio::null(),
                            })
                            .stdout(match self.verbose {
                                true => process::Stdio::inherit(),
                                false => process::Stdio::null(),
                            })
                            .env("PWD", path.display().to_string())
                            .status()
                        {
                            Ok(status) => println!(
                                "{}::{}",
                                path.strip_prefix(&config.src).unwrap().display(),
                                status
                                    .code()
                                    .map(|c| match c {
                                        0 => "OK".into(),
                                        _ => {
                                            errors += 1;
                                            format!("E{}", c)
                                        }
                                    })
                                    .unwrap_or_else(|| {
                                        errors += 1;
                                        "NONE".into()
                                    })
                            ),
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
