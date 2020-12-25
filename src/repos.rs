use crate::Location;
use std::path::{Path, PathBuf};
use std::{io, iter};

type GenericIter = Box<dyn Iterator<Item = PathBuf>>;

pub fn iter_repos(dir: &Path) -> io::Result<GenericIter> {
    match dir.join(".git").exists() {
        true => Ok(Box::new(iter::once(dir.into()))),
        false => match dir.read_dir() {
            Ok(dir_iter) => Ok(Box::new(
                dir_iter
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| path.is_dir())
                    .filter_map(|dir| iter_repos(&dir).ok())
                    .flatten(),
            )),
            Err(err) => Err(io::Error::new(
                err.kind(),
                format!("could not access '{}': {}", dir.display(), err),
            )),
        },
    }
}

pub fn iter_repos_matching(
    dir: &Path,
    targets: Vec<Location>,
    regex: bool,
) -> io::Result<GenericIter> {
    Ok(Box::new(iter_repos(dir)?.filter(move |path| {
        targets.len() == 0
            || targets.iter().any(|location| match regex {
                true => location.matches_re(&path),
                false => location.matches(&path),
            })
    })))
}

pub fn iter_repos_matching_exists(
    dir: &Path,
    targets: Vec<Location>,
    regex: bool,
) -> io::Result<GenericIter> {
    let mut repos = iter_repos_matching(dir, targets, regex)?;
    match repos.next() {
        Some(first) => Ok(Box::new(iter::once(first).chain(repos))),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no matching entities found",
        )),
    }
}

pub fn unique_repo(dir: &Path, target: Location, regex: bool) -> io::Result<PathBuf> {
    let mut repos = iter_repos_matching(dir, vec![target.clone()], regex)?;
    let repo = repos.next();
    match repos.next() {
        Some(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "multiple entities match query",
        )),
        None => match repo {
            Some(repo) => Ok(repo),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no entities match query"),
            )),
        },
    }
}
