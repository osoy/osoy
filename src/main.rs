#[macro_use]
mod cli;

mod config;
mod exec;
mod location;
mod operator;
mod repos;

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
