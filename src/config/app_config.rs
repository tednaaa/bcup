use crate::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
	pub destination: Destination,
	pub paths: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Destination {
	Telegram,
}

impl Default for AppConfig {
	fn default() -> Self {
		Self {
			destination: Destination::Telegram,
			paths: Vec::new(),
		}
	}
}

impl AppConfig {
	pub fn load() -> Result<Self> {
		let config_path = Self::config_dir()?.join("config.toml");
		let mut file = File::open(config_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		Ok(toml::from_str(&contents)?)
	}

	pub fn save(&self) -> Result<()> {
		let config_dir = Self::config_dir()?;
		fs::create_dir_all(&config_dir)?;

		let config_path = config_dir.join("config.toml");
		let toml = toml::to_string(self)?;
		fs::write(config_path, toml)?;
		Ok(())
	}

	fn config_dir() -> Result<PathBuf> {
		Ok(ProjectDirs::from("com", "bcup", "bcup")
			.ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
			.config_dir()
			.to_path_buf())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::tempdir;

	#[test]
	fn test_config_roundtrip() {
		let temp_dir = tempdir().unwrap();
		let config_path = temp_dir.path().join("config.toml");

		let original = AppConfig {
			destination: Destination::Telegram,
			paths: vec![PathBuf::from("/test/path")],
		};

		let toml = toml::to_string(&original).unwrap();
		std::fs::write(&config_path, toml).unwrap();

		let loaded = AppConfig::load().unwrap();
		assert_eq!(loaded.destination, original.destination);
		assert_eq!(loaded.paths, original.paths);
	}
}
