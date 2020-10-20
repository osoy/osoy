use dirs::home_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct DataDirectories {
    pub packages: PathBuf,
    pub bin: PathBuf,
}

pub fn data_dirs() -> Result<DataDirectories, String> {
    match home_dir() {
        Some(home) => {
            let osoy_path = home.join(".osoy");
            let data = DataDirectories {
                packages: osoy_path.join("packages"),
                bin: osoy_path.join("bin"),
            };

            if !create_dir_all(&data.packages).is_ok() {
                return Err(format!("could not create packages directory"));
            }

            if !create_dir_all(&data.bin).is_ok() {
                return Err(format!("could not create bin directory"));
            }

            Ok(data)
        }
        None => Err(format!("home directory not found")),
    }
}
