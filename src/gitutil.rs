use crate::config;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{
    Cred, CredentialType, Error, FetchOptions, Progress, RemoteCallbacks, Repository, Status,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{stdout, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct AuthCache {
    ssh_password: String,
    ssh_tries: HashMap<String, ()>,
}

impl AuthCache {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(AuthCache {
            ssh_password: String::new(),
            ssh_tries: HashMap::new(),
        }))
    }

    fn credentials(
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

fn log_progress(id: impl Display, stat: &Progress) -> bool {
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

fn fetch_options<'cb>(id: &'cb str, auth_cache: &Arc<Mutex<AuthCache>>) -> FetchOptions<'cb> {
    let mut callbacks = RemoteCallbacks::new();
    {
        let id = id.clone();
        let auth_cache = auth_cache.clone();
        callbacks.credentials(move |_, username, allowed_types| {
            auth_cache
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

    options
}

pub fn log(status: impl Display, id: impl Display) {
    println!("{:>9} {}", status, id)
}

pub fn clone(
    path: &Path,
    id: &str,
    url: &str,
    auth_cache: &Arc<Mutex<AuthCache>>,
) -> Result<Repository, Error> {
    RepoBuilder::new()
        .fetch_options(fetch_options(id, auth_cache))
        .clone(url, &path)
}

pub fn pull(path: &Path, id: &str, auth_cache: &Arc<Mutex<AuthCache>>) -> Result<(), Error> {
    let repo = Repository::open(path)?;

    let mut remote = repo.find_remote("origin")?;
    let mut head = repo.head()?;

    if !head.is_branch() {
        return Err(Error::from_str("head is not branch"));
    }

    let branch = String::from_utf8_lossy(head.shorthand_bytes()).to_string();

    remote.fetch(&[&branch], Some(&mut fetch_options(id, auth_cache)), None)?;

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

fn status_wt_char(st: &Status) -> Option<&'static char> {
    match st {
        s if s.contains(Status::WT_NEW) => Some(&'n'),
        s if s.contains(Status::WT_MODIFIED) => Some(&'m'),
        s if s.contains(Status::WT_DELETED) => Some(&'d'),
        s if s.contains(Status::WT_RENAMED) => Some(&'r'),
        s if s.contains(Status::WT_TYPECHANGE) => Some(&'t'),
        _ => None,
    }
}

fn status_index_char(st: &Status) -> Option<&'static char> {
    match st {
        s if s.contains(Status::INDEX_NEW) => Some(&'N'),
        s if s.contains(Status::INDEX_MODIFIED) => Some(&'M'),
        s if s.contains(Status::INDEX_DELETED) => Some(&'D'),
        s if s.contains(Status::INDEX_RENAMED) => Some(&'R'),
        s if s.contains(Status::INDEX_TYPECHANGE) => Some(&'T'),
        _ => None,
    }
}

pub struct RepoStatus {
    pub changes: Option<Vec<(char, String)>>,
    pub branch: Option<String>,
    pub graph: Option<(usize, usize)>,
}

impl RepoStatus {
    pub fn from(repo: &Repository) -> Self {
        let head = repo.head().ok();

        let (graph, branch) = head
            .map(|head| {
                let branch = String::from_utf8_lossy(head.shorthand_bytes()).to_string();
                (
                    repo.find_reference(&format!("refs/remotes/origin/{}", &branch))
                        .ok()
                        .map(|remote_ref| {
                            repo.reference_to_annotated_commit(&remote_ref)
                                .ok()
                                .map(|remote_commit| {
                                    repo.graph_ahead_behind(
                                        head.target().unwrap(),
                                        remote_commit.id(),
                                    )
                                    .ok()
                                })
                                .flatten()
                        })
                        .flatten(),
                    Some(branch),
                )
            })
            .unwrap_or((None, None));

        let changes = repo.statuses(None).ok().map(|statuses| {
            let (staged, unstaged): (Vec<_>, Vec<_>) = statuses
                .iter()
                .map(|entry| {
                    let st = entry.status();
                    let fname = String::from_utf8_lossy(entry.path_bytes());
                    (
                        status_index_char(&st).map(|ch| (*ch, fname.to_string())),
                        status_wt_char(&st).map(|ch| (*ch, fname.to_string())),
                    )
                })
                .unzip();
            staged
                .iter()
                .chain(unstaged.iter())
                .filter_map(|line| match line {
                    Some(line) => Some(line.clone()),
                    None => None,
                })
                .collect()
        });

        Self {
            changes,
            branch,
            graph,
        }
    }
}
