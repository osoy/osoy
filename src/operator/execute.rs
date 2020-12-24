use crate::{repos, Config, Exec, Location, StructOpt};
use std::env::set_current_dir;
use std::process;

#[derive(StructOpt, Debug, Clone)]
#[structopt(aliases = &["ex", "exec"], about = "Execute command in a repository")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    pub regex: bool,
    #[structopt(short, long, help = "Print command instead of executing")]
    pub print: bool,
    #[structopt(help = Location::about())]
    pub target: Location,
    #[structopt(help = "Command to execute in the repository")]
    pub command: String,
    #[structopt(help = "Arguments for the command")]
    pub arguments: Vec<String>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repos::iter_repos_matching(&config.src, vec![self.target.clone()], self.regex) {
            Ok(repos) => {
                let mut found_match = false;
                for path in repos {
                    found_match = true;
                    match self.print {
                        false => match set_current_dir(&path) {
                            Ok(_) => match process::Command::new(&self.command)
                                .args(&self.arguments)
                                .status()
                            {
                                Ok(status) => info!(
                                    "{} exit: {}",
                                    self.command,
                                    status
                                        .code()
                                        .map(|c| c.to_string())
                                        .unwrap_or("none".into())
                                ),
                                Err(err) => info!("failed to execute '{}': {}", self.command, err),
                            },
                            Err(err) => info!("could not access '{}': {}", path.display(), err),
                        },
                        true => println!(
                            "cd {} && {} {}",
                            path.display(),
                            self.command,
                            self.arguments.join(" ")
                        ),
                    }
                }
                if !found_match {
                    info!("no entities match query '{}'", self.target);
                }
            }
            Err(err) => info!("could not access '{}': {}", config.src.display(), err),
        }
    }
}
