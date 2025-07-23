use anyhow::Result;
use chrono::TimeDelta;
use serenity::all::{
	CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedValue, Timestamp,
};

pub async fn timeoutme(ctx: Context, mut interaction: CommandInteraction) -> Result<()> {
	let message = CreateInteractionResponseMessage::new().content("User muted themself.");
	let response = CreateInteractionResponse::Message(message);

	let ResolvedValue::String(time) = interaction
		.data
		.options()
		.iter()
		.find(|opt| opt.name == "time")
		.unwrap()
		.value
	else {
		unreachable!()
	};

	interaction
		.member
		.as_mut()
		.unwrap()
		.disable_communication_until_datetime(
			&ctx,
			Timestamp::now()
				.checked_add_signed(TimeDelta::seconds(time.parse::<i64>()?))
				.unwrap()
				.into(),
		)
		.await?;

	interaction.create_response(&ctx, response).await?;

	Ok(())
}
