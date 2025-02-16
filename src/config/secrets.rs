use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Secrets {
	pub telegram_bot_token: Option<String>,
}

impl Secrets {
	pub fn load() -> Result<Self> {
		let secrets_path = Self::secrets_path()?;
		let mut file = std::fs::File::open(secrets_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		Ok(toml::from_str(&contents)?)
	}

	pub fn save(&self) -> Result<()> {
		let secrets_path = Self::secrets_path()?;
		let toml = toml::to_string(self)?;
		std::fs::write(secrets_path, toml)?;
		Ok(())
	}

	fn secrets_path() -> Result<std::path::PathBuf> {
		Ok(AppConfig::config_dir()?.join("secrets.toml"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::tempdir;

	#[test]
	fn test_secrets_roundtrip() {
		let temp_dir = tempdir().unwrap();
		let secrets_path = temp_dir.path().join("secrets.toml");

		let original = Secrets {
			telegram_bot_token: Some("test_token".into()),
		};

		let toml = toml::to_string(&original).unwrap();
		std::fs::write(&secrets_path, toml).unwrap();

		let loaded = Secrets::load().unwrap();
		assert_eq!(loaded.telegram_bot_token, original.telegram_bot_token);
	}
}
