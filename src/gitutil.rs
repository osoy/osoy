use crate::config;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{
    Cred, CredentialType, Error, FetchOptions, Progress, RemoteCallbacks, Repository, Status,
};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};

#[derive(Default)]
struct AuthCache {
    ssh_password: String,
    ssh_tries: HashMap<PathBuf, String>,
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

            if self.ssh_tries.get(path) == Some(&self.ssh_password) {
                self.ssh_password = ask_secret!("password for '{}':", key_path.display());
            }
            self.ssh_tries
                .insert(path.into(), self.ssh_password.clone());

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

#[derive(Copy, Clone)]
pub struct ProgressObjects {
    pub total: usize,
    pub received: usize,
    pub indexed: usize,
}

impl Default for ProgressObjects {
    fn default() -> Self {
        Self {
            total: 1,
            received: 0,
            indexed: 0,
        }
    }
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
            let space = 50;
            let (left, mid, right, part) = match self.total {
                0 => (0, 0, space, 0),
                _ => {
                    let indexed = space * self.indexed / self.total;
                    let received = space * self.received / self.total;
                    let lesser = min(indexed, received);
                    let greater = max(indexed, received);
                    (
                        lesser,
                        greater - lesser,
                        space - greater,
                        100 * min(self.indexed, self.received) / self.total,
                    )
                }
            };
            write!(
                stdout,
                "\r[{}{}{}] {}%",
                "=".repeat(left),
                "-".repeat(mid),
                "·".repeat(right),
                part
            )
            .ok()
            .map(|_| stdout.flush());
            write!(stdout, "\r{}", termion::clear::CurrentLine).ok();
        }
    }
}

pub enum FetchMessage {
    Progress(ProgressObjects),
    Done(
        (
            PathBuf,
            Result<(Repository, String), Error>,
            ProgressObjects,
        ),
    ),
}

#[derive(Clone)]
struct Fetch {
    limit: usize,
    auth_cache: Arc<Mutex<AuthCache>>,
    threads: Arc<RwLock<Vec<JoinHandle<()>>>>,
    queue: Arc<Mutex<Vec<Sender<()>>>>,
    progress: Arc<RwLock<HashMap<PathBuf, ProgressObjects>>>,
    sender: Sender<FetchMessage>,
}

impl Fetch {
    fn new(limit: usize) -> (Self, Receiver<FetchMessage>) {
        let (sender, receiver) = channel();
        (
            Self {
                limit,
                auth_cache: Arc::new(Mutex::new(AuthCache::default())),
                threads: Arc::new(RwLock::new(vec![])),
                queue: Arc::new(Mutex::new(vec![])),
                progress: Arc::new(RwLock::new(HashMap::new())),
                sender: sender,
            },
            receiver,
        )
    }

    fn credentials(
        &self,
        path: &Path,
        username: Option<&str>,
        allowed_types: CredentialType,
    ) -> Result<Cred, Error> {
        self.auth_cache
            .lock()
            .unwrap()
            .credentials(path, username, allowed_types)
    }

    fn progress(&self) -> ProgressObjects {
        self.progress
            .read()
            .unwrap()
            .values()
            .fold(ProgressObjects::default(), |acc, v| acc + *v)
    }

    fn update_progress(&self, path: &Path, prog: ProgressObjects) -> bool {
        match self.progress.try_write() {
            Ok(mut progress) => progress.insert(path.to_path_buf(), prog),
            Err(_) => return false,
        };
        self.sender
            .send(FetchMessage::Progress(self.progress()))
            .is_ok()
    }

    fn init_progress(&self, path: PathBuf) {
        self.progress
            .write()
            .map(|mut progress| progress.insert(path, ProgressObjects::default()))
            .ok();
    }

    fn complete_progress(&self, path: &Path) {
        self.progress
            .write()
            .map(|mut progress| {
                progress.get_mut(path).map(|mut prog| {
                    prog.received = prog.total;
                    prog.indexed = prog.total;
                })
            })
            .ok();
    }

    fn wait_slot(&self) {
        if self.threads.read().unwrap().len() >= self.limit {
            let (sender, receiver) = channel();
            self.queue.lock().unwrap().push(sender);
            receiver.recv().ok();
        }
    }

    fn free_slot(&self) {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() > 0 {
            queue.remove(0).send(()).ok();
        }
    }

    fn wait_and_spawn<F>(&self, path: PathBuf, f: F)
    where
        F: FnOnce() -> Result<(Repository, String), Error> + Send + 'static,
    {
        self.wait_slot();
        let fetch = self.clone();
        self.threads.write().unwrap().push(spawn(move || {
            fetch.init_progress(path.clone());
            let res = f();
            fetch.complete_progress(&path);
            fetch
                .sender
                .send(FetchMessage::Done((path, res, fetch.progress())))
                .ok();
            fetch.free_slot();
        }));
    }
}

fn fetch_options<'cb>(path: &'cb Path, fetch: Fetch) -> FetchOptions<'cb> {
    let mut callbacks = RemoteCallbacks::new();
    {
        let path = path.clone();
        let fetch = fetch.clone();
        callbacks.credentials(move |_, username, allowed_types| {
            fetch.credentials(path, username, allowed_types)
        });
    }
    {
        let path = path.clone();
        callbacks.transfer_progress(move |prog| fetch.update_progress(path, prog.into()));
    }

    let mut options = FetchOptions::new();
    options.remote_callbacks(callbacks);

    options
}

fn pull_one(path: &Path, fetch: Fetch, overwrite: bool) -> Result<(Repository, String), Error> {
    let repo = Repository::open(path)?;
    let mut message = "done";

    {
        let mut remote = repo.find_remote("origin")?;
        let mut head = repo.head()?;

        if !head.is_branch() {
            return Err(Error::from_str("head is not branch"));
        }

        let branch = String::from_utf8_lossy(head.shorthand_bytes()).to_string();

        remote.fetch(&[&branch], Some(&mut fetch_options(path, fetch)), None)?;

        let fetch_commit =
            repo.reference_to_annotated_commit(&repo.find_reference("FETCH_HEAD")?)?;

        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            message = "up-to-date";
        } else if analysis.0.is_fast_forward() {
            head.set_target(fetch_commit.id(), "pull: Fast-forward")?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
        } else if analysis.0.is_normal() {
            if overwrite {
                head.set_target(fetch_commit.id(), "pull: Overwrite")?;
                repo.checkout_head(Some(
                    CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))?;
            } else {
                return Err(Error::from_str(
                    "Merge is unimplemented but you can use flag --force to overwrite",
                ));
            }
        } else {
            return Err(Error::from_str("Unimplemented scenario"));
        }
    }

    Ok((repo, message.into()))
}

pub fn pull(paths: Vec<PathBuf>, threads: usize, overwrite: bool) -> Receiver<FetchMessage> {
    let (fetch, receiver) = Fetch::new(threads);

    spawn(move || {
        for path in paths {
            let fetch_clone = fetch.clone();
            fetch.wait_and_spawn(path.clone(), move || {
                pull_one(&path, fetch_clone, overwrite)
            });
        }
    });

    receiver
}

pub fn clone(url_path_pairs: Vec<(String, PathBuf)>, threads: usize) -> Receiver<FetchMessage> {
    let (fetch, receiver) = Fetch::new(threads);

    spawn(move || {
        for (url, path) in url_path_pairs {
            let fetch_clone = fetch.clone();
            fetch.wait_and_spawn(path.clone(), move || {
                RepoBuilder::new()
                    .fetch_options(fetch_options(&path, fetch_clone))
                    .clone(&url, &path)
                    .map(|repo| (repo, "done".into()))
            });
        }
    });

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
