use crate::config;
use git2::{Cred, CredentialType, Error};

pub fn credentials(
    id: &str,
    username: Option<&str>,
    allowed_types: CredentialType,
) -> Result<Cred, Error> {
    if allowed_types.is_ssh_key() {
        let key_path = config::home_path(".ssh/id_rsa").unwrap();
        let pubkey_path = config::home_path(".ssh/id_rsa.pub").unwrap();
        Cred::ssh_key(
            &match username {
                Some(name) => name.into(),
                None => ask_string!("username for '{}':", &id),
            },
            Some(&pubkey_path),
            &key_path,
            Some(&ask_secret!(
                "enter passphrase for '{}':",
                key_path.display()
            )),
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
