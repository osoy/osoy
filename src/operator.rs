use crate::{Config, Exec, StructOpt};
use structopt::clap::AppSettings;

macro_rules! operator {
    ($($oper:tt),*) => {
        $(
            mod $oper;
        )*

        #[derive(StructOpt, Debug)]
        #[structopt(
            about = "Manage git repositories",
            global_settings = &[
                AppSettings::VersionlessSubcommands,
                AppSettings::ColorNever,
            ],
        )]
        pub enum Operator {
            $(
                #[allow(non_camel_case_types)]
                $oper {
                    #[structopt(flatten)]
                    opt: $oper::Opt,
                },
            )*
        }

        impl Exec for Operator {
            fn exec(self, config: Config) -> i32 {
                match self {
                    $(
                        Operator::$oper { opt } => opt.exec(config),
                    )*
                }
            }
        }
    }
}

operator!(
    clone,
    completions,
    execute,
    link,
    list,
    locate,
    new,
    pull,
    remove,
    rename,
    unlink
);
