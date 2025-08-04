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
	let webhook = CreateCommandOption::new(CommandOptionType::SubCommand, "webhook", "Gets a webhook for your blog");

	let name = CreateCommandOption::new(CommandOptionType::String, "name", "The new name")
		.min_length(2)
		.max_length(32);

	let rename = CreateCommandOption::new(CommandOptionType::SubCommand, "rename", "Renames your blog")
		.set_sub_options(vec![name]);

	let blog = CreateCommand::new("blog")
		.description("Commands related to blog management")
		.set_options(vec![create, delete, rename, webhook]);

	let duration = CreateCommandOption::new(CommandOptionType::Integer, "duration", "The duration in hours")
		.min_int_value(1)
		.max_int_value(24)
		.required(true);

	let me = CreateCommandOption::new(CommandOptionType::SubCommand, "me", "Times you out for a few hours")
		.set_sub_options(vec![duration]);

	let timeout = CreateCommand::new("timeout")
		.description("Commands related to timeout management")
		.set_options(vec![me]);

	guild_id.set_commands(&http, vec![blog, timeout]).await?;

	Ok(())
}
