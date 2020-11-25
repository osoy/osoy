#[macro_use]
mod out;

mod config;
mod location;
mod operate;
mod opt;
mod util;

pub use config::Config;
pub use location::Location;
use operate::operate;
use opt::{Opt, StructOpt};

fn main() {
    let opt = Opt::from_args();
    let config = Config::from_env();
    operate(opt, config);
}
