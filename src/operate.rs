use crate::opt::{Operator, Opt, StructOpt};
use crate::{util, Config};

pub fn operate(opt: Opt, config: Config) {
    match opt.operator {
        Operator::Completions { shell } => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
        }

        Operator::List { targets, regex } => match util::iter_repos(&config.src) {
            Ok(iter) => iter
                .filter(|path| {
                    targets.len() == 0
                        || targets.iter().any(|location| match regex {
                            true => location.matches_re(&path),
                            false => location.matches(&path),
                        })
                })
                .for_each(|path| {
                    path.strip_prefix(&config.src)
                        .ok()
                        .map(|rel| println!("{}", rel.display()));
                }),
            Err(err) => info!("could not access '{}': {}", config.src.display(), err),
        },

        Operator::New { targets } => {
            for location in targets {
                let path = config.src.join(location.id());
                match path.exists() {
                    true => info!("entity '{}' already exists", path.display()),
                    false => match git2::Repository::init(&path) {
                        Ok(repo) => match repo.remote("origin", &location.url()) {
                            Ok(_) => {}
                            Err(err) => info!("could not set remote: {}", err),
                        },
                        Err(err) => info!("could not init: {}", err),
                    },
                }
            }
        }

        _ => todo!(),
    }
}
