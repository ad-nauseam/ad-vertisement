use std::time::SystemTime;

use anyhow::Result;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, Timestamp};

pub async fn timeoutme(ctx: Context, mut interaction: CommandInteraction) -> Result<()> {
	let Some(option) = interaction.data.options.first() else {
		anyhow::bail!("No options present");
	};

	let Some(duration) = option.value.as_i64() else {
		anyhow::bail!("Option is not an integer");
	};

	let Some(member) = interaction.member.as_deref_mut() else {
		anyhow::bail!("Command not used in a guild");
	};

	let seconds = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
	let timestamp = Timestamp::from_unix_timestamp(seconds as i64 + duration)?;

	let content = match member.disable_communication_until_datetime(&ctx, timestamp).await {
		Ok(()) => format!("<@{}> is now muted until <t:{}:t>", member.user.id, timestamp.unix_timestamp()),
		Err(error) => format!("Timeout not successful: {error}"),
	};

	let message = CreateInteractionResponseMessage::new().content(content);
	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(&ctx, response).await?;

	Ok(())
}
