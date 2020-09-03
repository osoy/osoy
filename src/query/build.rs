use std::path::Path;

pub enum BuildMethod {
    Make,
    Cargo,
}

pub fn get_build_method(dir: &Path) -> Option<BuildMethod> {
    if dir.join("Makefile").is_file() || dir.join("makefile").is_file() {
        return Some(BuildMethod::Make);
    } else if dir.join("Cargo.toml").is_file() {
        return Some(BuildMethod::Cargo);
    }
    None
}
