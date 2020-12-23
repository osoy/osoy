use super::Operator;
use crate::{Config, Exec, StructOpt};
use structopt::clap::Shell;

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "Generate shell completion script")]
pub struct Opt {
    #[structopt(possible_values = &Shell::variants())]
    pub shell: Shell,
}

impl Exec for Opt {
    fn exec(self, _: Config) {
        Operator::clap().gen_completions_to(
            env!("CARGO_PKG_NAME"),
            self.shell,
            &mut std::io::stdout(),
        );
    }
}
