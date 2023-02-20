use crate::identity::Identity;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir, File};
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub identity: Vec<IdentityConfig>,
}

impl Config {
    pub fn account_for_url(&self, service: &str, url: &str) -> anyhow::Result<Identity> {
        let candidate_identities: Vec<Identity> = self
            .identity
            .iter()
            .flat_map(|ic| {
                let inner: Vec<Identity> = ic
                    .accounts_for_url(service, url)
                    .iter()
                    .map(|ac| Identity::from(ic, ac))
                    .collect();
                inner
            })
            .collect();

        match candidate_identities.len() {
            0 => Err(anyhow!("No identity found for URL - {}", url)),
            1 => Ok(candidate_identities.first().unwrap().clone()),
            _ => Err(anyhow!(
                "Multiple identities found for URL - {:?}",
                candidate_identities
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentityConfig {
    pub id: String,
    pub user: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub account: Option<Vec<AccountConfig>>,
}

impl IdentityConfig {
    pub fn identity_for_service(&self, service: &str) -> anyhow::Result<Option<Identity>> {
        if let Some(account) = &self.account {
            let accounts: Vec<&AccountConfig> =
                account.iter().filter(|ac| ac.service == service).collect();

            match accounts.len() {
                0 => Ok(None),
                1 => Ok(Some(Identity::from(self, accounts.first().unwrap()))),
                _ => Err(anyhow!(
                    "No account for service {} in identity {}",
                    service,
                    self
                )),
            }
        } else {
            Ok(None)
        }
    }

    fn accounts_for_url(&self, service: &str, url: &str) -> Vec<&AccountConfig> {
        if let Some(account) = &self.account {
            account
                .iter()
                .filter(|a| a.service == service && a.account_matches_url(url))
                .collect()
        } else {
            vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountConfig {
    pub service: String,
    pub user: Option<String>,
    pub match_url: Option<String>,
    pub description: Option<String>,
    pub token: Option<String>,
}

impl AccountConfig {
    fn account_matches_url(&self, url: &str) -> bool {
        match &self.match_url {
            Some(match_url) => {
                if match_url.ends_with('*') {
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

pub struct LazyConfig {
    config: Option<Config>,
}

impl LazyConfig {
    pub fn new() -> Self {
        LazyConfig { config: None }
    }

    pub fn required(&mut self) -> anyhow::Result<()> {
        if self.config.is_none() {
            self.config = Some(load_config()?)
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn update<F>(&mut self, change: F) -> anyhow::Result<()>
    where
        F: Fn(Config) -> anyhow::Result<Config>,
    {
        self.config = Some(update_config(change)?);

        Ok(())
    }
}

impl Deref for LazyConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.config.as_ref().unwrap()
    }
}

fn load_config() -> anyhow::Result<Config> {
    let config_path = get_config_path()?;

    let mut f = File::open(&config_path)
        .with_context(|| format!("Failed to open config at - {:?}", config_path))?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;
    toml::from_str::<Config>(&content).with_context(|| "Invalid config file content")
}

#[allow(dead_code)]
fn update_config<F>(change: F) -> anyhow::Result<Config>
where
    F: Fn(Config) -> anyhow::Result<Config>,
{
    let config_path = get_config_path()?;

    let mut f = File::open(&config_path)
        .with_context(|| format!("Failed to open config at - {:?}", config_path))?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let config =
        toml::from_str::<Config>(&content).with_context(|| "Invalid config file content")?;

    let updated_config = change(config)?;

    let mut f = File::create(&config_path)?;
    f.write_all(toml::to_string(&updated_config)?.as_bytes())?;

    Ok(updated_config)
}

fn get_config_path() -> anyhow::Result<PathBuf> {
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

    Ok(config_path)
}

fn create_default_config(config_path: &PathBuf) -> anyhow::Result<()> {
    let mut f = File::create(config_path)
        .with_context(|| format!("Failed to open config at - {:?}", config_path))?;
    let new_config = Config {
        version: "1.0".to_string(),
        identity: vec![],
    };
    let content = toml::to_string(&new_config).with_context(|| {
        format!(
            "Failed to create default config file content - {:?}",
            config_path
        )
    })?;
    f.write_all(content.as_bytes()).with_context(|| {
        format!(
            "Failed to write default content to new config file - {:?}",
            config_path
        )
    })?;

    Ok(())
}

pub fn verify_config(config: &mut LazyConfig) -> anyhow::Result<()> {
    config.required()?;

    let unique_ids: HashSet<&str> = config.identity.iter().map(|ic| ic.id.as_str()).collect();
    if unique_ids.len() != config.identity.len() {
        return Err(anyhow!("Identities must have a unique id"));
    }

    Ok(())
}

impl Display for IdentityConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            self.id,
            self.email.as_ref().unwrap_or(&"No email".to_string())
        )
    }
}
