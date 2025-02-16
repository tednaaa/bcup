use crate::Result;
use teloxide::prelude::*;

pub struct TelegramClient;

impl TelegramClient {
	pub async fn handle_backup_request(&self, msg: Message) -> Result<()> {
		if !self.validate_user(&msg).await {
			self.bot.send_message(msg.chat.id, "⛔ Unauthorized access!").await?;
			return Ok(());
		}

		// Only proceed if user is authorized
		let archive = Archiver::create_archive(&config.paths)?;
		let temp_path = archive.into_temp_path();

		self.send_backup(&temp_path).await?;
		Ok(())
	}
}
