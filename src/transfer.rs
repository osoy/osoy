use crate::config;
use git2::{Cred, CredentialType, Error, Progress};
use std::collections::HashMap;
use std::io::{stdout, Write};

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

pub fn log_progress(id: &str, stat: &Progress) -> bool {
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
