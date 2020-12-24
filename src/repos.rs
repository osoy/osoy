use crate::Location;
use std::path::{Path, PathBuf};
use std::{io, iter};

pub fn iter_repos(dir: &Path) -> io::Result<Box<dyn Iterator<Item = PathBuf>>> {
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
) -> io::Result<Box<dyn Iterator<Item = PathBuf>>> {
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
        Ok(repos) => {
            let mut repo = None;
            for path in repos {
                if repo.is_none() {
                    repo = Some(path);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "multiple entities match query",
                    ));
                }
            }
            match repo {
                Some(repo) => Ok(repo),
                None => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("no entries match query '{}'", target),
                )),
            }
        }
        Err(err) => Err(io::Error::new(
            err.kind(),
            format!("could not access '{}': {}", dir.display(), err),
        )),
    }
}
