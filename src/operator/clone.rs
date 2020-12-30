use crate::{config, Config, Exec, Location, StructOpt};
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

#[derive(StructOpt, Debug)]
#[structopt(about = "Clone from remote repositories")]
pub struct Opt {
    #[structopt(short, long, help = "Print what is being done")]
    verbose: bool,
    #[structopt(required = true, min_values = 1, help = Location::about())]
    targets: Vec<Location>,
}

impl Exec for Opt {
    fn exec(self, config: Config) {
        let cached_ssh_password = Arc::new(Mutex::new(String::new()));

        for location in self.targets {
            let id = location.id();
            let path = config.src.join(&id);
            if path.exists() {
                info!("entity '{}' already exists", &id)
            } else {
                let mut callbacks = RemoteCallbacks::new();

                let ssh_password =
                    Arc::new(Mutex::new(cached_ssh_password.lock().unwrap().clone()));

                {
                    let id = id.clone();
                    let mut tried = false;
                    let ssh_password = ssh_password.clone();

                    callbacks.credentials(move |_, username, allowed_types| {
                        if allowed_types.is_ssh_key() {
                            let key_path = config::home_path(".ssh/id_rsa").unwrap();
                            let pubkey_path = config::home_path(".ssh/id_rsa.pub").unwrap();

                            let mut ssh_password = ssh_password.lock().unwrap();
                            if tried {
                                *ssh_password =
                                    ask_secret!("password for '{}':", key_path.display());
                            }
                            tried = true;

                            Cred::ssh_key(
                                &match username {
                                    Some(name) => name.into(),
                                    None => ask_string!("username for '{}':", &id),
                                },
                                Some(&pubkey_path),
                                &key_path,
                                Some(&ssh_password.clone()),
                            )
                        } else if allowed_types.is_user_pass_plaintext() {
                            Cred::userpass_plaintext(
                                &ask_string!("username for '{}':", &id),
                                &ask_secret!("password for '{}':", &id),
                            )
                        } else {
                            unimplemented!()
                        }
                    });
                }

                {
                    let id = id.clone();
                    callbacks.transfer_progress(move |stats| {
                        let total = stats.total_objects();
                        let recieved = stats.received_objects();
                        let indexed = stats.indexed_objects();
                        eprint!(
                            "{:3}% {:3}% {}\r",
                            100 * recieved / total,
                            100 * indexed / total,
                            id,
                        );
                        stdout().flush().ok();
                        true
                    });
                }

                let mut options = FetchOptions::new();
                options.remote_callbacks(callbacks);

                let res = RepoBuilder::new()
                    .fetch_options(options)
                    .clone(&location.url(), &path);

                print!("\u{1b}[K");

                match res {
                    Ok(_) => {
                        *cached_ssh_password.lock().unwrap() = ssh_password.lock().unwrap().clone();
                        println!("{:>9} {}", "done", id);
                    }
                    Err(err) => println!(
                        "{:>9} {}{}",
                        "failed",
                        id,
                        match self.verbose {
                            false => "".into(),
                            true => format!("\n{:10}{}", "", err),
                        }
                    ),
                }
            }
        }
    }
}
