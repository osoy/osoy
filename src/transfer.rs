use crate::config;
use git2::build::CheckoutBuilder;
use git2::{
    AutotagOption, Cred, CredentialType, Error, FetchOptions, Progress, RemoteCallbacks, Repository,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{stdout, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Cache {
    ssh_password: String,
    ssh_tries: HashMap<String, ()>,
}

pub fn cache() -> Cache {
    Cache {
        ssh_password: String::new(),
        ssh_tries: HashMap::new(),
    }
}

impl Cache {
    pub fn credentials(
        &mut self,
        id: &str,
        username: Option<&str>,
        allowed_types: CredentialType,
    ) -> Result<Cred, Error> {
        if allowed_types.is_ssh_key() {
            let key_path = config::home_path(".ssh/id_rsa").unwrap();
            let pubkey_path = config::home_path(".ssh/id_rsa.pub").unwrap();

            if self.ssh_tries.contains_key(id) {
                self.ssh_password = ask_secret!("password for '{}':", key_path.display());
                self.ssh_tries.remove(id);
            } else {
                self.ssh_tries.insert(id.into(), ());
            }

            Cred::ssh_key(
                &match username {
                    Some(name) => name.into(),
                    None => ask_string!("username for '{}':", &id),
                },
                Some(&pubkey_path),
                &key_path,
                Some(&self.ssh_password.clone()),
            )
        } else if allowed_types.is_user_pass_plaintext() {
            Cred::userpass_plaintext(
                &ask_string!("username for '{}':", &id),
                &ask_secret!("password for '{}':", &id),
            )
        } else {
            unimplemented!()
        }
    }
}

pub fn log_progress(id: impl Display, stat: &Progress) -> bool {
    let total = stat.total_objects();
    let recieved = stat.received_objects();
    let indexed = stat.indexed_objects();
    eprint!(
        "{:3}% {:3}% {}\r",
        100 * recieved / total,
        100 * indexed / total,
        id,
    );
    stdout().flush().ok();
    true
}

pub fn log(status: impl Display, id: impl Display) {
    println!("{:>9} {}", status, id)
}

pub fn pull(path: &Path, id: &str, cache: &Arc<Mutex<Cache>>) -> Result<(), Error> {
    let repo = Repository::open(path)?;
    let mut callbacks = RemoteCallbacks::new();
    {
        let id = id.clone();
        let cache = cache.clone();
        callbacks.credentials(move |_, username, allowed_types| {
            cache
                .lock()
                .unwrap()
                .credentials(&id, username, allowed_types)
        });
    }
    {
        let id = id.clone();
        callbacks.transfer_progress(move |stat| log_progress(&id, &stat));
    }

    let mut options = FetchOptions::new();
    options.remote_callbacks(callbacks);
    options.download_tags(AutotagOption::All);

    let mut remote = repo.find_remote("origin")?;
    let mut head = repo.head()?;

    if !head.is_branch() {
        return Err(Error::from_str("head is not branch"));
    }

    let branch = String::from_utf8_lossy(head.shorthand_bytes()).to_string();

    remote.fetch(&[&branch], Some(&mut options), None)?;

    let fetch_commit = repo.reference_to_annotated_commit(&repo.find_reference("FETCH_HEAD")?)?;

    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        head.set_target(fetch_commit.id(), "pull: Fast-forward")?;
        repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
    } else if analysis.0.is_normal() {
        return Err(Error::from_str("merge unimplemented"));
    }

    Ok(())
}
