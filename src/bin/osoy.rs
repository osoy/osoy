use osoy::{Config, Exec, Operator};
use std::{env, process};
use structopt::StructOpt;

fn main() {
    match Operator::from_args_safe() {
        Ok(operator) => process::exit(operator.exec(Config::from_env())),
        Err(err) => {
            let args = env::args().skip(1).collect::<Vec<String>>();
            args.get(0)
                .map(|exe| {
                    process::Command::new(&format!("{}-{}", env!("CARGO_PKG_NAME"), exe))
                        .args(&args[1..])
                        .status()
                        .ok()
                })
                .flatten()
                .map(|status| status.code())
                .flatten()
                .map(process::exit);
            err.exit()
        }
    }
}
