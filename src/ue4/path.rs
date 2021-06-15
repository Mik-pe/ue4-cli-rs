use std::{
    io::{self, Result},
    path::{Path, PathBuf},
};

use serde_json::Value;
use walkdir::WalkDir;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

pub struct UE4Project {
    pub version_string: String,
}

pub struct UE4PathFinder {
    pub engine_root: PathBuf,
}

impl UE4PathFinder {
    pub fn new(project: UE4Project) -> io::Result<UE4PathFinder> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        //The windows regkey for a ue4 installation
        let version_regkey = format!(
            "SOFTWARE\\EpicGames\\Unreal Engine\\{}",
            project.version_string,
        );
        let cur_ver = hklm.open_subkey(version_regkey)?;
        let installed_dir: String = cur_ver.get_value("InstalledDirectory")?;
        Ok(UE4PathFinder {
            engine_root: PathBuf::from(installed_dir),
        })
    }
}

impl UE4Project {
    pub fn new(uproject_file: &Path) -> Self {
        let file = std::fs::File::open(uproject_file).unwrap();
        let reader = std::io::BufReader::new(file);
        let json_data: Value = serde_json::from_reader(reader).unwrap();
        let version_string = String::from(json_data["EngineAssociation"].as_str().unwrap());
        UE4Project { version_string }
    }

    pub fn guess_from_dir(project_dir: &Path) -> Result<Self> {
        for entry in WalkDir::new(project_dir).into_iter().filter_map(|e| e.ok()) {
            let file_name = entry.file_name().to_string_lossy();

            if file_name.ends_with(".uproject") {
                return Ok(Self::new(entry.path()));
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find any .uproject files in dir",
        ))
    }
}
