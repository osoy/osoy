use crate::Location;
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
pub struct Opt {
    #[structopt(subcommand)]
    pub operator: Operator,
}

#[derive(StructOpt, Debug)]
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
