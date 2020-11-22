use std::path::PathBuf;
use std::{env, fs, io};

#[cfg(target_family = "unix")]
const HOME_VAR: &str = "HOME";
#[cfg(target_family = "windows")]
const HOME_VAR: &str = "USERPROFILE";

const OSOY_HOME_VAR: &str = "OSOY_HOME";

pub struct Config {
    home: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let home = {
            let env_osoy_home = env::var(OSOY_HOME_VAR).unwrap_or("".into());
            match env_osoy_home.len() {
                0 => {
                    let env_home = env::var(HOME_VAR).unwrap();
                    match env_home.len() {
                        0 => panic!("HOME variable not set"),
                        _ => PathBuf::from(env_home).join(".osoy"),
                    }
                }
                _ => env_osoy_home.into(),
            }
        };

        Self { home }
    }

    pub fn get_src(&self) -> io::Result<PathBuf> {
        let src = self.home.join("src");
        fs::create_dir_all(&src)?;
        Ok(src)
    }

    pub fn get_bin(&self) -> io::Result<PathBuf> {
        let bin = self.home.join("bin");
        fs::create_dir_all(&bin)?;
        Ok(bin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home() {
        let osoy_home = "/home/user/.local/share/osoy";
        env::set_var(OSOY_HOME_VAR, osoy_home);
        assert_eq!(Config::from_env().home, PathBuf::from(osoy_home));

        let home = "/home/user";
        env::set_var(OSOY_HOME_VAR, "");
        env::set_var(HOME_VAR, home);
        assert_eq!(Config::from_env().home, PathBuf::from(home).join(".osoy"));
    }
}
