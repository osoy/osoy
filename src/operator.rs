use crate::{Config, Exec};
use structopt::clap::AppSettings;
use structopt::StructOpt;

macro_rules! operator {
    ($($oper:tt),*$(,)?) => {
        $(
            pub mod $oper;
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
                $oper($oper::Opt),
            )*
        }

        impl Exec for Operator {
            fn exec(self, config: Config) -> i32 {
                match self {
                    $(
                        Operator::$oper(opt) => opt.exec(config),
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
    unlink,
);
