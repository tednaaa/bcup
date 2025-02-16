mod commands;
pub use commands::*;

pub fn run() -> crate::Result<()> {
	let cli = Cli::from_args();
	cli.execute()
}
