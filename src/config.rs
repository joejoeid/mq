use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub database_path: Option<PathBuf>,
    pub pager: Option<String>,
    pub browser: Option<String>,
    pub max_results: Option<usize>,
}

impl Config {
    pub fn load(custom_path: Option<&str>) -> anyhow::Result<Self> {
        let path = if let Some(p) = custom_path {
            PathBuf::from(p)
        } else if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "mq") {
            proj_dirs.config_dir().join("config.toml")
        } else {
            return Ok(Config::default());
        };

        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}
