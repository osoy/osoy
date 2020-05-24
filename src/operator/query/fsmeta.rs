use std::os::unix::fs::{symlink, MetadataExt};
use std::path::Path;

pub fn is_exe(path: &Path) -> bool {
    if let Ok(meta) = path.metadata() {
        if meta.mode() & 0o100 > 0 {
            return true;
        }
    }
    false
}

pub fn is_symlink(path: &Path) -> bool {
    if let Ok(meta) = path.symlink_metadata() {
        if meta.file_type().is_symlink() {
            return true;
        }
    }
    false
}

pub fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    symlink(target, link)?;
    Ok(())
}
