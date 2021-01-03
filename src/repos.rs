use crate::Location;
use std::path::{Path, PathBuf};
use std::{fs, io, iter};

/// Get recursive iterator over git repositories in given directory.
pub fn iterate(dir: &Path) -> io::Result<Box<dyn Iterator<Item = PathBuf>>> {
    match dir.join(".git").exists() {
        true => Ok(Box::new(iter::once(dir.into()))),
        false => match dir.read_dir() {
            Ok(dir_iter) => Ok(Box::new(
                dir_iter
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| path.is_dir())
                    .filter_map(|dir| iterate(&dir).ok())
                    .flatten(),
            )),
            Err(err) => Err(io::Error::new(
                err.kind(),
                match err.kind() {
                    io::ErrorKind::NotFound => format!("no repositories found"),
                    _ => format!("could not access '{}': {}", dir.display(), err),
                },
            )),
        },
    }
}

/// Get iterator over repositories that match any of the given targets.
pub fn iterate_matching(
    dir: &Path,
    targets: Vec<Location>,
    regex: bool,
) -> io::Result<Box<dyn Iterator<Item = PathBuf>>> {
    Ok(Box::new(iterate(dir)?.filter(move |path| {
        targets.len() == 0
            || targets.iter().any(|location| match regex {
                true => location.matches_re(&path),
                false => location.matches(&path),
            })
    })))
}

/// Same as `iterate_matching` except returns error if no matching repositories found.
pub fn iterate_matching_exists(
    dir: &Path,
    targets: Vec<Location>,
    regex: bool,
) -> io::Result<Box<dyn Iterator<Item = PathBuf>>> {
    let mut repos = iterate_matching(dir, targets, regex)?;
    match repos.next() {
        Some(first) => Ok(Box::new(iter::once(first).chain(repos))),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "no matching entities found",
        )),
    }
}

/// Get an unique repository in directory otherwise return error.
pub fn unique(dir: &Path, target: Location, regex: bool) -> io::Result<PathBuf> {
    let mut repos = iterate_matching(dir, vec![target.clone()], regex)?;
    let repo = repos.next();
    match repos.next() {
        Some(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "multiple entities match query",
        )),
        None => match repo {
            Some(repo) => Ok(repo),
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "no entities match query",
            )),
        },
    }
}

/// Remove empty directories recursively returning remove count.
fn remove_dir_rec(dir: &Path) -> usize {
    match fs::remove_dir(dir) {
        Ok(_) => match dir.parent() {
            Some(parent) => remove_dir_rec(parent) + 1,
            None => 1,
        },
        Err(_) => 0,
    }
}

/// Remove directory and parent directories if empty returning count of removed parent directories.
pub fn remove(dir: &Path) -> io::Result<usize> {
    let res = fs::remove_dir_all(dir);
    let count = dir
        .parent()
        .map(|parent| remove_dir_rec(parent))
        .unwrap_or(0);
    res.map(|_| count)
}

/// Rename directory and remove previous parent directories if empty.
pub fn rename(target: &Path, dest: &Path) -> io::Result<usize> {
    match dest.exists() {
        true => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "destination entity already exists",
        )),
        false => {
            dest.parent().map(|parent| fs::create_dir_all(parent));
            match fs::rename(target, dest) {
                Ok(_) => Ok(target
                    .parent()
                    .map(|parent| remove_dir_rec(parent))
                    .unwrap_or(0)),
                Err(err) => {
                    dest.parent().map(|parent| remove_dir_rec(parent));
                    Err(err)
                }
            }
        }
    }
}
