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

	let blog = CreateCommand::new("blog")
		.description("Commands related to blog management")
		.set_options(vec![create, delete]);

	guild_id.set_commands(&http, vec![blog]).await?;

	Ok(())
}
