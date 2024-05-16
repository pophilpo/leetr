use crate::errors::ConfigError;
use crate::project_generator::ProjectType;
use serde::Deserialize;
use toml;

use std::error;

use std::fs;

#[derive(Deserialize)]
pub struct ConfigFile {
    default_lang: String,
}

impl ConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        // TODO: Generic path
        let file_content = fs::read_to_string("/home/philipp/.config/leetr/leetr.toml")?;

        Ok(toml::from_str(&file_content)?)
    }
}

#[derive(Debug)]
pub struct Config {
    pub default_lang: ProjectType,
}

impl From<ConfigFile> for Config {
    fn from(config_file: ConfigFile) -> Self {
        let default_lang = ProjectType::from(config_file.default_lang);

        Config { default_lang }
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config_file = ConfigFile::new()?;
        Ok(Config::from(config_file))
    }
}
