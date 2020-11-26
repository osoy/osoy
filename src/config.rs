use std::env;
use std::path::PathBuf;

#[cfg(target_family = "unix")]
const HOME_VAR: &str = "HOME";
#[cfg(target_family = "windows")]
const HOME_VAR: &str = "USERPROFILE";

const OSOY_HOME_VAR: &str = "OSOY_HOME";

#[derive(Debug, Clone)]
pub struct Config {
    pub src: PathBuf,
    pub bin: PathBuf,
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

        Self {
            src: home.join("src"),
            bin: home.join("bin"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home() {
        let osoy_home = "/home/user/.local/share/osoy";
        env::set_var(OSOY_HOME_VAR, osoy_home);
        assert_eq!(Config::from_env().src, PathBuf::from(osoy_home).join("src"));

        let home = "/home/user";
        env::set_var(OSOY_HOME_VAR, "");
        env::set_var(HOME_VAR, home);
        assert_eq!(
            Config::from_env().bin,
            PathBuf::from(home).join(".osoy/bin")
        );
    }
}
