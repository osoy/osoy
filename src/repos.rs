use crate::Location;
use std::path::{Path, PathBuf};
use std::{io, iter};

type GenericIter = Box<dyn Iterator<Item = PathBuf>>;

pub fn iter_repos(dir: &Path) -> io::Result<GenericIter> {
    Ok(match dir.join(".git").exists() {
        true => Box::new(iter::once(dir.into())),
        false => Box::new(
            dir.read_dir()?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .filter_map(|dir| iter_repos(&dir).ok())
                .flatten(),
        ),
    })
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

pub fn unique_repo(dir: &Path, target: Location, regex: bool) -> io::Result<PathBuf> {
    match iter_repos_matching(dir, vec![target.clone()], regex) {
        Ok(mut repos) => {
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
                        format!("no entries match query '{}'", target),
                    )),
                },
            }
        }
        Err(err) => Err(io::Error::new(
            err.kind(),
            format!("could not access '{}': {}", dir.display(), err),
        )),
    }
}
