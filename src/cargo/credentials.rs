use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoCredentials {
    pub registry: CargoRegistryCredentials
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoRegistryCredentials {
    pub token: String
}

pub fn get_current_credentials() -> anyhow::Result<CargoCredentials> {
    let credentials_path = get_credentials_path()?;

    let mut f = File::open(&credentials_path).with_context(|| format!("Failed to open Cargo credentials file - {:?}", credentials_path))?;
    let mut content = String::new();
    f.read_to_string(&mut content).with_context(|| format!("Failed to read Cargo credentials file - {:?}", credentials_path))?;

    let credentials = toml::from_str::<CargoCredentials>(content.as_str()).with_context(|| "Failed to deserialize the Cargo credentials file content")?;

    Ok(credentials)
}

pub fn write_credentials(cargo_credentials: CargoCredentials) -> anyhow::Result<()> {
    let credentials_path = get_credentials_path()?;

    let credentials = toml::to_string(&cargo_credentials).with_context(|| "Failed to serialize the Cargo credentials file content")?;

    let mut f = File::create(&credentials_path).with_context(|| format!("Failed to open Cargo credentials file - {:?}", credentials_path))?;
    f.write_all(credentials.as_bytes()).with_context(|| "Failed to write Cargo credentials file")?;

    Ok(())
}

fn get_credentials_path() -> anyhow::Result<PathBuf> {
    let home_path = match home::home_dir() {
        Some(p) => p,
        None => return Err(anyhow!("Unable to find your home directory")),
    };

    let credentials_path = home_path.join(".cargo").join("credentials");
    if !credentials_path.exists() {
        return Err(anyhow!("Cargo credentials file not found. Please log in with `cargo login` for the first time, then use `identity switch`"));
    }

    Ok(credentials_path)
}
