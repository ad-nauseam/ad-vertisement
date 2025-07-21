use anyhow::Result;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage};

pub async fn create(ctx: Context, interaction: CommandInteraction) -> Result<()> {
	let message = CreateInteractionResponseMessage::new().content("Used `/blog create`");
	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(&ctx, response).await?;

	Ok(())
}

pub async fn delete(ctx: Context, interaction: CommandInteraction) -> Result<()> {
	let message = CreateInteractionResponseMessage::new().content("Used `/blog delete`");
	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(&ctx, response).await?;

	Ok(())
}
