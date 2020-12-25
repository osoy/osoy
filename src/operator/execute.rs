use crate::{repos, Config, Exec, Location, StructOpt};
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
        match repos::iter_repos_matching_exists(&config.src, vec![self.target.clone()], self.regex)
        {
            Ok(repos) => {
                for path in repos {
                    match set_current_dir(&path) {
                        Ok(_) => match process::Command::new(&self.command)
                            .args(&self.arguments)
                            .status()
                        {
                            Ok(status) => {
                                if self.verbose {
                                    info!(
                                        "{} exit: {}",
                                        self.command,
                                        status
                                            .code()
                                            .map(|c| c.to_string())
                                            .unwrap_or("none".into())
                                    );
                                }
                            }
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
