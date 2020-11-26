mod completions;
mod execute;
mod list;
mod new;

use crate::{Config, Exec, StructOpt};
use structopt::clap::AppSettings;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Manage git repositories",
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColorNever,
    ],
)]
pub enum Operator {
    Completions {
        #[structopt(flatten)]
        opt: completions::Completions,
    },

    List {
        #[structopt(flatten)]
        opt: list::List,
    },

    New {
        #[structopt(flatten)]
        opt: new::New,
    },

    Execute {
        #[structopt(flatten)]
        opt: execute::Execute,
    },
}

impl Exec for Operator {
    fn exec(self, config: Config) {
        match self {
            Operator::Completions { opt } => opt.exec(config),
            Operator::List { opt } => opt.exec(config),
            Operator::New { opt } => opt.exec(config),
            Operator::Execute { opt } => opt.exec(config),
        }
    }
}
