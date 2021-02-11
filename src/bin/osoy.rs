use osoy::{Config, Exec, Operator};
use structopt::StructOpt;

fn main() {
    match Operator::from_args_safe() {
        Ok(operator) => {
            let config = Config::from_env();
            std::process::exit(operator.exec(config))
        }
        Err(err) => err.exit(),
    }
}
