mod completions;
mod execute;
mod list;
mod locate;
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
    Execute {
        #[structopt(flatten)]
        opt: execute::Execute,
    },
    Locate {
        #[structopt(flatten)]
        opt: locate::Locate,
    },

    New {
        #[structopt(flatten)]
        opt: new::New,
    },
}

use Operator::*;

impl Exec for Operator {
    fn exec(self, config: Config) {
        match self {
            Completions { opt } => opt.exec(config),
            List { opt } => opt.exec(config),
            New { opt } => opt.exec(config),
            Execute { opt } => opt.exec(config),
            Locate { opt } => opt.exec(config),
        }
    }
}
