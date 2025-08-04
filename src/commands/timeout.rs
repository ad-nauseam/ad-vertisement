use anyhow::Result;
use serenity::all::{
	CommandDataOption, CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
	EditMember, InteractionResponseFlags, Timestamp,
};

pub async fn me(ctx: &Context, interaction: &CommandInteraction, options: &[CommandDataOption]) -> Result<()> {
	let duration = options
		.first()
		.and_then(|option| option.value.as_i64())
		.ok_or_else(|| anyhow::anyhow!("Duration is not present"))?;

	let member = interaction
		.member
		.as_deref()
		.ok_or_else(|| anyhow::anyhow!("Command was not used in a guild"))?;

	let seconds = Timestamp::now().unix_timestamp() + duration * 3600;
	let timestamp = Timestamp::from_unix_timestamp(seconds)?;

	let builder = EditMember::new()
		.audit_log_reason("Used timeout command")
		.disable_communication_until_datetime(timestamp);

	member.guild_id.edit_member(ctx, member.user.id, builder).await?;

	let message = CreateInteractionResponseMessage::new()
		.content(format!("<@{}> is now muted until <t:{}:t>", member.user.id, seconds))
		.flags(InteractionResponseFlags::SUPPRESS_NOTIFICATIONS);

	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(ctx, response).await?;

	Ok(())
}
