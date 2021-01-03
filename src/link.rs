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
        .filter_map(|link| link.read_link().map(|dest| (link, dest)).ok())
        .filter(move |(_, dest)| repos.iter().any(|repo| dest.starts_with(repo))))
}

pub fn create(bin: &Path, exe: &Path) -> io::Result<PathBuf> {
    let link_path = bin.join(exe.file_name().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "no file name found",
    ))?);
    {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::fs::symlink;
            symlink(exe, &link_path)
        }
        #[cfg(target_family = "windows")]
        {
            use std::os::windows::fs::symlink_file;
            symlink_file(exe, &link_path)
        }
    }
    .map(|_| link_path)
}

fn is_executable(path: &Path) -> bool {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::MetadataExt;
        !path.read_link().unwrap_or(path.into()).is_dir()
            && path
                .metadata()
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

pub fn executables(dir: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| is_executable(path)))
}
