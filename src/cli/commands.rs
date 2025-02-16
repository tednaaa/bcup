use crate::{
	backup::Archiver,
	config::{AppConfig, Secrets},
	telegram::TelegramClient,
	Result,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bcup", about = "Simple backup CLI")]
pub enum Cli {
	#[structopt(about = "Run backup")]
	Run,

	Init {
		#[structopt(long)]
		bot_token: String,

		#[structopt(long)]
		user_id: i64,
	},

	#[structopt(about = "Configure settings")]
	Config {
		#[structopt(help = "Config key (destination|telegram_bot_token|add_path)")]
		key: String,

		#[structopt(help = "Config value")]
		value: Option<String>,
	},
}

impl Cli {
	fn execute(&self) -> Result<()> {
		match self {
			Cli::Init { bot_token, user_id } => {
				let mut secrets = Secrets::load().unwrap_or_default();
				secrets.telegram_bot_token = Secret::new(bot_token.clone());
				secrets.authorized_user_id = *user_id;
				secrets.save()?;
				println!("Security configuration saved!");
			} // ... existing handlers
		}
	}
	fn run_backup(&self) -> Result<()> {
		let config = AppConfig::load()?;
		let secrets = Secrets::load()?;

		match config.destination {
			crate::config::Destination::Telegram => {
				let bot_token = secrets
					.telegram_bot_token
					.ok_or_else(|| anyhow::anyhow!("Telegram bot token not configured"))?;

				let archive = Archiver::create_archive(&config.paths)?;
				let temp_path = archive.into_temp_path();

				tokio::runtime::Runtime::new()?
					.block_on(async { TelegramClient::send_file(&bot_token, &temp_path).await })?;
			}
		}
		Ok(())
	}

	fn handle_config(&self, key: &str, value: &Option<String>) -> Result<()> {
		match key {
			"destination" => self.handle_destination(value),
			"telegram_bot_token" => self.handle_telegram_token(value),
			"add_path" => self.handle_add_path(value),
			_ => Err(anyhow::anyhow!("Unknown config key")),
		}
	}

	fn handle_destination(&self, value: &Option<String>) -> Result<()> {
		let dest = value
			.as_ref()
			.ok_or_else(|| anyhow::anyhow!("Missing destination value"))?;
		match dest.as_str() {
			"telegram" => {
				let mut config = AppConfig::load().unwrap_or_default();
				config.destination = crate::config::Destination::Telegram;
				config.save()?;
			}
			_ => return Err(anyhow::anyhow!("Invalid destination")),
		}
		Ok(())
	}

	fn handle_telegram_token(&self, value: &Option<String>) -> Result<()> {
		let token = value.as_ref().ok_or_else(|| anyhow::anyhow!("Missing token value"))?;
		let mut secrets = Secrets::load().unwrap_or_default();
		secrets.telegram_bot_token = Some(token.clone());
		secrets.save()?;
		Ok(())
	}

	fn handle_add_path(&self, value: &Option<String>) -> Result<()> {
		let path = value.as_ref().ok_or_else(|| anyhow::anyhow!("Missing path value"))?;
		let mut config = AppConfig::load()?;
		config.paths.push(path.into());
		config.save()?;
		Ok(())
	}
}
