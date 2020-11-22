mod config;
mod location;
mod opt;
mod util;

use config::Config;
pub use location::Location;
use opt::*;

fn main() {
    let config = Config::from_env();
    let opt = Opt::from_args();

    match opt.operator {
        Operator::Completions { shell } => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
        }

        Operator::List { targets, regex } => match util::iter_repos(&config.src) {
            Ok(iter) => iter
                .filter(|repo| {
                    targets.len() == 0
                        || targets.iter().any(|location| match regex {
                            true => location.matches_re(&repo),
                            false => location.matches(&repo),
                        })
                })
                .for_each(|repo| {
                    repo.strip_prefix(&config.src)
                        .ok()
                        .map(|rel| println!("{}", rel.display()));
                }),
            Err(err) => eprintln!("{}", err),
        },

        _ => todo!(),
    }
}
