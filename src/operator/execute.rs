use crate::{repo, Config, Exec, Location, StructOpt};
use std::env::set_current_dir;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(aliases = &["ex", "exec"], about = "Execute command in a repository")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(help = Location::about())]
    target: Location,
    #[structopt(help = "Command to execute in the repository")]
    command: String,
    #[structopt(help = "Arguments for the command")]
    arguments: Vec<String>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repo::iterate_matching_exists(&config.src, vec![self.target], self.regex) {
            Ok(iter) => {
                for path in iter {
                    match set_current_dir(&path) {
                        Ok(_) => match process::Command::new(&self.command)
                            .args(&self.arguments)
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
                                        _ => format!("E{}", c),
                                    })
                                    .unwrap_or("NONE".into())
                            ),
                            Err(err) => info!("failed to execute '{}': {}", self.command, err),
                        },
                        Err(err) => info!("could not access '{}': {}", path.display(), err),
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}
