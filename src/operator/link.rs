use crate::{link, repo, Config, Exec, Location, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(alias = "ln", about = "Create symbolic links for executables")]
pub struct Opt {
    #[structopt(short, long, help = "Use regular expressions")]
    regex: bool,
    #[structopt(short, long, help = "Do not prompt")]
    force: bool,
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        match repo::iterate_matching_exists(&config.src, self.targets, self.regex) {
            Ok(iter) => {
                for path in iter {
                    if let Ok(exe_iter) = link::executables(&path) {
                        for exe in exe_iter.filter(|exe| {
                            link::link_path(&config.bin, exe)
                                .map(|sym| &link::deref_rec(&sym) != exe)
                                .unwrap_or(false)
                        }) {
                            let exe_display = exe.strip_prefix(&config.src).unwrap().display();
                            if self.force || ask_bool!("link '{}'?", exe_display) {
                                match link::create(&config.bin, &exe) {
                                    Ok(sym) => {
                                        if self.verbose {
                                            info!(
                                                "'{}' -> '{}'",
                                                sym.strip_prefix(&config.bin).unwrap().display(),
                                                exe_display
                                            );
                                        }
                                    }
                                    Err(err) => {
                                        info!("could not link '{}': {}", path.display(), err);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => info!("{}", err),
        }
    }
}