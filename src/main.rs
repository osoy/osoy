#[macro_use]
mod out;

mod config;
mod location;
mod operator;
mod util;

pub use config::Config;
pub use location::Location;
use operator::{Operator, StructOpt};

fn main() {
    let operator = Operator::from_args();
    let config = Config::from_env();
    operator.exec(&config);
}
