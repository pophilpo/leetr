use crate::errors::ConfigError;
use crate::project_generator::ProjectType;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigFile {
    default_lang: String,
}

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
    pub fn new(lang: String) -> Result<Self, ConfigError> {
        //let config_file = ConfigFile::new()?;
        //Ok(Config::from(config_file))

        let project_type = ProjectType::from(lang);

        Ok(Self {
            default_lang: project_type,
        })
    }
}
