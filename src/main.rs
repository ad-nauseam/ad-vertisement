#![deny(clippy::all, clippy::nursery)]

mod blogs;
mod commands;
mod handler;

use std::env;

use anyhow::Result;
use serenity::all::{ActivityData, Client, GatewayIntents};

use crate::handler::Handler;

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv()?;

	let token = env::var("TOKEN")?;
	let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;

	let mut client = Client::builder(token, intents)
		.activity(ActivityData::playing("Playing around with my balls"))
		.event_handler(Handler)
		.await?;

	client.start().await?;

	Ok(())
}
