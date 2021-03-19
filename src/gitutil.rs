use crate::config;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{
    Cred, CredentialType, Error, FetchOptions, Progress, RemoteCallbacks, Repository, Status,
};
use std::collections::{HashMap, HashSet};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct AuthCache {
    ssh_password: String,
    ssh_tries: HashSet<PathBuf>,
}

impl AuthCache {
    fn credentials(
        &mut self,
        path: &Path,
        username: Option<&str>,
        allowed_types: CredentialType,
    ) -> Result<Cred, Error> {
        if allowed_types.is_ssh_key() {
            let key_path = config::home_path(".ssh/id_rsa").unwrap();
            let pubkey_path = config::home_path(".ssh/id_rsa.pub").unwrap();

            if self.ssh_tries.contains(path) {
                self.ssh_password = ask_secret!("password for '{}':", key_path.display());
                self.ssh_tries.remove(path);
            } else {
                self.ssh_tries.insert(path.into());
            }

            Cred::ssh_key(
                &match username {
                    Some(name) => name.into(),
                    None => ask_string!("username for '{}':", path.display()),
                },
                Some(&pubkey_path),
                &key_path,
                Some(&self.ssh_password),
            )
        } else if allowed_types.is_user_pass_plaintext() {
            Cred::userpass_plaintext(
                &ask_string!("username for '{}':", path.display()),
                &ask_secret!("password for '{}':", path.display()),
            )
        } else {
            unimplemented!()
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct ProgressObjects {
    pub total: usize,
    pub received: usize,
    pub indexed: usize,
}

impl From<Progress<'_>> for ProgressObjects {
    fn from(prog: Progress<'_>) -> Self {
        Self {
            total: prog.total_objects(),
            received: prog.received_objects(),
            indexed: prog.indexed_objects(),
        }
    }
}

impl std::ops::Add for ProgressObjects {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
            received: self.received + other.received,
            indexed: self.indexed + other.indexed,
        }
    }
}

impl ProgressObjects {
    pub fn print(&self) {
        let stdout = stdout();
        if termion::is_tty(&stdout) {
            let mut stdout = stdout.lock();
            write!(
                stdout,
                "\r({:8}:{:8})/{:8}",
                self.indexed, self.received, self.total
            )
            .ok()
            .map(|_| stdout.flush());
            write!(stdout, "\r{}", termion::clear::CurrentLine).ok();
        }
    }
}

pub type Done = Result<Repository, Error>;

pub enum FetchMessage {
    Progress(ProgressObjects),
    Done((PathBuf, Done)),
}

#[derive(Clone)]
struct Fetch {
    progress: Arc<Mutex<HashMap<PathBuf, ProgressObjects>>>,
    sender: Sender<FetchMessage>,
}

impl Fetch {
    fn new() -> (Self, Receiver<FetchMessage>) {
        let (sender, receiver) = channel();
        (
            Self {
                progress: Arc::new(Mutex::new(HashMap::new())),
                sender: sender,
            },
            receiver,
        )
    }

    fn post_progress(&self, path: &Path, prog: ProgressObjects) -> bool {
        let mut progress = match self.progress.try_lock() {
            Ok(progress) => progress,
            Err(_) => return false,
        };
        progress.insert(path.to_path_buf(), prog);
        self.sender
            .send(FetchMessage::Progress(
                progress
                    .values()
                    .fold(ProgressObjects::default(), |acc, v| acc + *v),
            ))
            .is_ok()
    }

    fn post_done(&self, path: &Path, done: Done) {
        self.progress
            .lock()
            .ok()
            .map(|mut progress| progress.remove(path));
        self.sender
            .send(FetchMessage::Done((path.into(), done)))
            .ok();
    }
}

fn fetch_options<'cb>(
    path: &'cb Path,
    auth_cache: Arc<Mutex<AuthCache>>,
    fetch: Fetch,
) -> FetchOptions<'cb> {
    let mut callbacks = RemoteCallbacks::new();
    {
        let path = path.clone();
        callbacks.credentials(move |_, username, allowed_types| {
            auth_cache
                .lock()
                .unwrap()
                .credentials(path, username, allowed_types)
        });
    }
    {
        let path = path.clone();
        callbacks.transfer_progress(move |prog| fetch.post_progress(path, prog.into()));
    }

    let mut options = FetchOptions::new();
    options.remote_callbacks(callbacks);

    options
}

fn pull_one(
    path: &Path,
    auth_cache: Arc<Mutex<AuthCache>>,
    fetch: Fetch,
) -> Result<Repository, Error> {
    let repo = Repository::open(path)?;

    {
        let mut remote = repo.find_remote("origin")?;
        let mut head = repo.head()?;

        if !head.is_branch() {
            return Err(Error::from_str("head is not branch"));
        }

        let branch = String::from_utf8_lossy(head.shorthand_bytes()).to_string();

        remote.fetch(
            &[&branch],
            Some(&mut fetch_options(path, auth_cache, fetch)),
            None,
        )?;

        let fetch_commit =
            repo.reference_to_annotated_commit(&repo.find_reference("FETCH_HEAD")?)?;

        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_fast_forward() {
            head.set_target(fetch_commit.id(), "pull: Fast-forward")?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
        } else if analysis.0.is_normal() {
            return Err(Error::from_str("merge unimplemented"));
        }
    }

    Ok(repo)
}

pub fn pull<I>(paths: I) -> Receiver<FetchMessage>
where
    I: Iterator<Item = PathBuf>,
{
    let auth_cache = Arc::new(Mutex::new(AuthCache::default()));
    let (fetch, receiver) = Fetch::new();

    for path in paths {
        let auth_cache = auth_cache.clone();
        let fetch = fetch.clone();
        thread::spawn(move || {
            fetch
                .clone()
                .post_done(&path, pull_one(&path, auth_cache, fetch))
        });
    }

    receiver
}

pub type UrlPathPair = (String, PathBuf);

pub fn clone<I>(url_path_pairs: I) -> Receiver<FetchMessage>
where
    I: Iterator<Item = UrlPathPair>,
{
    let auth_cache = Arc::new(Mutex::new(AuthCache::default()));
    let (fetch, receiver) = Fetch::new();

    for (url, path) in url_path_pairs {
        let auth_cache = auth_cache.clone();
        let fetch = fetch.clone();
        thread::spawn(move || {
            fetch.clone().post_done(
                &path,
                RepoBuilder::new()
                    .fetch_options(fetch_options(&path, auth_cache, fetch))
                    .clone(&url, &path),
            );
        });
    }

    receiver
}

fn status_wt_char(st: &Status) -> Option<char> {
    match st {
        s if s.contains(Status::WT_NEW) => Some('n'),
        s if s.contains(Status::WT_MODIFIED) => Some('m'),
        s if s.contains(Status::WT_DELETED) => Some('d'),
        s if s.contains(Status::WT_RENAMED) => Some('r'),
        s if s.contains(Status::WT_TYPECHANGE) => Some('t'),
        _ => None,
    }
}

fn status_index_char(st: &Status) -> Option<char> {
    match st {
        s if s.contains(Status::INDEX_NEW) => Some('N'),
        s if s.contains(Status::INDEX_MODIFIED) => Some('M'),
        s if s.contains(Status::INDEX_DELETED) => Some('D'),
        s if s.contains(Status::INDEX_RENAMED) => Some('R'),
        s if s.contains(Status::INDEX_TYPECHANGE) => Some('T'),
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

        let (graph, branch) = head.map_or((None, None), |head| {
            let branch = String::from_utf8_lossy(head.shorthand_bytes()).to_string();
            (
                repo.find_reference(&format!("refs/remotes/origin/{}", &branch))
                    .ok()
                    .map(|remote_ref| {
                        repo.reference_to_annotated_commit(&remote_ref)
                            .ok()
                            .map(|remote_commit| {
                                repo.graph_ahead_behind(head.target().unwrap(), remote_commit.id())
                                    .ok()
                            })
                            .flatten()
                    })
                    .flatten(),
                Some(branch),
            )
        });

        let changes = repo.statuses(None).ok().map(|statuses| {
            let (staged, unstaged): (Vec<_>, Vec<_>) = statuses
                .iter()
                .map(|entry| {
                    let st = entry.status();
                    let fname = String::from_utf8_lossy(entry.path_bytes());
                    (
                        status_index_char(&st).map(|ch| (ch, fname.to_string())),
                        status_wt_char(&st).map(|ch| (ch, fname.to_string())),
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
