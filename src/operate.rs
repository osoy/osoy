use crate::opt::{Operator, Opt, StructOpt};
use crate::{util, Config};

pub fn operate(opt: Opt, config: Config) {
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
            Err(err) => info!("could not access '{}': {}", config.src.display(), err),
        },

        _ => todo!(),
    }
}
