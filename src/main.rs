#[macro_use]
mod out;

mod config;
mod exec;
mod location;
mod operator;
mod util;

use config::Config;
use exec::Exec;
use location::Location;
use operator::Operator;
use structopt::StructOpt;

fn main() {
    let operator = Operator::from_args();
    let config = Config::from_env();
    operator.exec(config);
}
