use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub identity: Vec<IdentityConfig>,
}

impl Config {
    pub fn identities_for_url(&self, url: &str) -> Vec<&IdentityConfig> {
        self.identity
            .iter()
            .filter(|ic| ic.matches_url(url))
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityConfig {
    pub id: String,
    pub email: Option<String>,
    pub match_url: Option<String>,
    pub description: Option<String>,
}

impl IdentityConfig {
    pub fn matches_url(&self, url: &str) -> bool {
        match &self.match_url {
            Some(match_url) => {
                if match_url.ends_with("*") {
                    let mut url_prefix = match_url.clone();
                    url_prefix.pop();

                    url.contains(&url_prefix)
                } else {
                    url == match_url
                }
            }
            None => false,
        }
    }
}

pub fn load_config() -> anyhow::Result<Config> {
    let home_path = match home::home_dir() {
        Some(p) => p,
        None => return Err(anyhow!("Unable to find your home directory")),
    };

    let config_dir = home_path.join(".config");
    if !config_dir.exists() {
        create_dir(&config_dir)
            .with_context(|| format!("Failed to create config directory - {:?}", config_dir))?;
    }

    let config_path = config_dir.join("identity.toml");
    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    let mut f = File::open(&config_path)
        .with_context(|| format!("Failed to open config at - {:?}", config_path))?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(toml::from_str::<Config>(&content).with_context(|| "Invalid config file content")?)
}

fn create_default_config(config_path: &PathBuf) -> anyhow::Result<()> {
    let mut f = File::create(config_path)
        .with_context(|| format!("Failed to open config at - {:?}", config_path))?;
    let new_config = Config {
        version: "1.0".to_string(),
        identity: vec![],
    };
    let content = toml::to_string(&new_config)
        .with_context(|| format!("Failed to create default config file content"))?;
    f.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write default content to new config file"))?;

    Ok(())
}