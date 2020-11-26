use crate::{util, Config, Location};
use structopt::clap::{AppSettings, Shell};
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Manage git repositories",
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColorNever,
    ],
)]
pub enum Operator {
    #[structopt(about = "Generate shell completion script")]
    Completions {
        #[structopt(possible_values = &Shell::variants())]
        shell: Shell,
    },

    #[structopt(alias = "ls", about = "List repositories")]
    List {
        #[structopt(short, long, help = "Use regular expressions")]
        regex: bool,
        #[structopt(help = Location::about())]
        targets: Vec<Location>,
    },

    #[structopt(alias = "n", about = "Create new empty git repositories")]
    New {
        #[structopt(required = true, min_values = 1, help = Location::about())]
        targets: Vec<Location>,
    },

    #[structopt(alias = "ex", about = "Execute command in git repository")]
    Execute {
        #[structopt(help = Location::about())]
        target: Location,
        #[structopt(help = "Command to execute in the repository")]
        command: String,
        #[structopt(help = "Arguments for the command")]
        arguments: Vec<String>,
    },
}

impl Operator {
    pub fn exec(self, config: &Config) {
        match self {
            Operator::Completions { shell } => {
                Operator::clap().gen_completions_to(
                    env!("CARGO_PKG_NAME"),
                    shell,
                    &mut std::io::stdout(),
                );
            }

            Operator::List { targets, regex } => match util::iter_repos(&config.src) {
                Ok(iter) => iter
                    .filter(|path| {
                        targets.len() == 0
                            || targets.iter().any(|location| match regex {
                                true => location.matches_re(&path),
                                false => location.matches(&path),
                            })
                    })
                    .for_each(|path| {
                        path.strip_prefix(&config.src)
                            .ok()
                            .map(|rel| println!("{}", rel.display()));
                    }),
                Err(err) => info!("could not access '{}': {}", config.src.display(), err),
            },

            Operator::New { targets } => {
                for location in targets {
                    let path = config.src.join(location.id());
                    match path.exists() {
                        true => info!("entity '{}' already exists", path.display()),
                        false => match git2::Repository::init(&path) {
                            Ok(repo) => match repo.remote("origin", &location.url()) {
                                Ok(_) => {}
                                Err(err) => info!("could not set remote: {}", err),
                            },
                            Err(err) => info!("could not init: {}", err),
                        },
                    }
                }
            }

            _ => todo!(),
        }
    }
}
