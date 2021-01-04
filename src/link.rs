use std::io;
use std::path::{Path, PathBuf};

pub fn iterate(
    bin: &Path,
    repos: Vec<PathBuf>,
) -> io::Result<impl Iterator<Item = (PathBuf, PathBuf)>> {
    Ok(bin
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter_map(|sym| sym.read_link().map(|dest| (sym, dest)).ok())
        .filter(move |(_, dest)| repos.iter().any(|repo| dest.starts_with(repo))))
}

pub fn link_path(bin: &Path, exe: &Path) -> io::Result<PathBuf> {
    exe.file_name()
        .map(|osname| bin.join(osname))
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "no file name found",
        ))
}

pub fn create(bin: &Path, exe: &Path) -> io::Result<PathBuf> {
    let sym = link_path(bin, exe)?;
    {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::fs::symlink;
            symlink(exe, &sym)
        }
        #[cfg(target_family = "windows")]
        {
            use std::os::windows::fs::symlink_file;
            symlink_file(exe, &sym)
        }
    }
    .map(|_| sym)
}

pub fn deref_rec(path: &Path) -> PathBuf {
    match path.read_link() {
        Ok(dest) => deref_rec(&match dest.is_relative() {
            false => dest,
            true => path.parent().unwrap().join(dest),
        }),
        Err(_) => path.into(),
    }
}

fn is_executable(path: &Path) -> bool {
    let path = deref_rec(path);
    path.is_file() && {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::fs::MetadataExt;
            path.metadata()
                .map(|meta| meta.mode() & 0o100 > 0)
                .unwrap_or(false)
        }
        #[cfg(target_family = "windows")]
        {
            path.extension()
                .map(|ostyp| ostyp.to_str().map(|typ| typ == "exe" || typ == "bat"))
                .flatten()
                .unwrap_or(false)
        }
    }
}

pub fn executables(dir: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| is_executable(path)))
}
