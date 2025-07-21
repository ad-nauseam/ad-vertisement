mod handler;

use std::env;

use anyhow::Result;
use handler::Handler;
use serenity::all::{ActivityData, Client, GatewayIntents};

#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv()?;

	let token = env::var("TOKEN")?;
	let intents = GatewayIntents::GUILD_MESSAGES;

	let mut client = Client::builder(token, intents)
		.activity(ActivityData::custom("around with my balls"))
		.event_handler(Handler)
		.await?;

	client.start().await?;

	Ok(())
}
