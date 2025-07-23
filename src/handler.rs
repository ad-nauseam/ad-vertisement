use serenity::all::{Context, EventHandler, Interaction, Message, Ready};

use crate::commands;

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		let Interaction::Command(command) = interaction else {
			return;
		};

		let result = match command.data.name.as_str() {
			"blog" => match command.data.options.first().map_or("", |option| &option.name) {
				"create" => commands::blog::create(ctx, command).await,
				"delete" => commands::blog::delete(ctx, command).await,
				name => Err(anyhow::anyhow!("Invalid blog subcommand: '{name}'")),
			},
			"timeoutme" => commands::timeoutme::timeoutme(ctx, command).await,
			name => Err(anyhow::anyhow!("Invalid command: '{name}'")),
		};

		if let Err(error) = result {
			eprintln!("{error}");
		}
	}

	async fn message(&self, ctx: Context, message: Message) {
		if message.mentions_me(&ctx).await.unwrap_or_default() {
			message.reply(&ctx, "Please use my slash commands!").await.ok();
		}
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is running!", ready.user.name);
	}
}
