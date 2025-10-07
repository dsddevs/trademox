use serde::Deserialize;
use serde_yaml;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YamlConfig {
    pub cors: CorsConfig,
}
impl YamlConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = fs::File::open(path)?;
        let config = serde_yaml::from_reader(&file)?;
        Ok(config)
    }
}
