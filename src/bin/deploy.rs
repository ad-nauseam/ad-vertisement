use std::env;

use anyhow::Result;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, GuildId, HttpBuilder};

#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv()?;

	let application_id = env::var("APPLICATION_ID")?.parse()?;
	let guild_id: GuildId = env::var("GUILD_ID")?.parse()?;

	let token = env::var("TOKEN")?;
	let http = HttpBuilder::new(token).application_id(application_id).build();

	let create = CreateCommandOption::new(CommandOptionType::SubCommand, "create", "Creates a new blog");
	let delete = CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "Deletes your blog");

	let timeoutme = CreateCommand::new("timeoutme")
		.description("Times you out")
		.set_options(vec![
			CreateCommandOption::new(CommandOptionType::String, "time", "Duration of time")
				.add_string_choice("1 hour", "3600")
				.add_string_choice("2 hours", "7200")
				.add_string_choice("3 hours", "10800")
				.add_string_choice("4 hours", "14400"),
		]);

	let blog = CreateCommand::new("blog")
		.description("Commands related to blog management")
		.set_options(vec![create, delete]);

	guild_id.set_commands(&http, vec![blog, timeoutme]).await?;

	Ok(())
}
