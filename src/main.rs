mod location;
pub use location::Location;

mod config;

mod opt;
use opt::*;

fn main() {
    let opt = Opt::from_args();
    match opt.operator {
        Operator::Completions { shell } => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
            std::process::exit(0);
        }
        _ => todo!(),
    }
}
