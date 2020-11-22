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
