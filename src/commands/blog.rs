use std::env;

use anyhow::Result;
use serenity::all::{
	CommandDataOptionValue, CommandInteraction, Context, CreateChannel, CreateInteractionResponse,
	CreateInteractionResponseMessage, EditChannel, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId,
};

pub async fn create(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let guild = &interaction.guild_id.unwrap();

	let channels = guild.channels(&ctx).await.unwrap();

	if channels.iter().any(|(_, channel)| {
		channel.topic.is_some() && channel.topic.as_ref().unwrap() == &interaction.user.id.to_string()
	}) {
		anyhow::bail!("Do not be greedy! Your blog channel already exists.");
	}

	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been created!");
	let response = CreateInteractionResponse::Message(message);

	let category = channels.iter().find(|(_, channel)| channel.name == "Blogs").unwrap().0;

	let channel = CreateChannel::new(&interaction.user.name)
		.category(category)
		.permissions(vec![
			PermissionOverwrite {
				allow: Permissions::SEND_MESSAGES,
				deny: Permissions::from_bits_retain(0),
				kind: PermissionOverwriteType::Member(interaction.user.id),
			},
			PermissionOverwrite {
				allow: Permissions::from_bits_retain(0),
				deny: Permissions::SEND_MESSAGES,
				kind: PermissionOverwriteType::Role(RoleId::new(env::var("GUILD_ID")?.parse::<u64>()?)),
			},
		])
		.topic(interaction.user.id.to_string());

	guild.create_channel(&ctx, channel).await.unwrap();

	interaction.create_response(&ctx, response).await?;

	Ok(())
}

pub async fn nick(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let Some(options) = interaction.data.options.first() else {
		anyhow::bail!("No options present")
	};

	let CommandDataOptionValue::SubCommand(subcommand) = &options.value else {
		anyhow::bail!("Option is not a subcommand")
	};

	let name = if let Some(option) = subcommand.first() {
		match option.value.as_str() {
			Some(duration) => duration,
			None => anyhow::bail!("Option is not a str"),
		}
	} else {
		&interaction.user.name
	};

	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been renamed!");
	let response = CreateInteractionResponse::Message(message);

	let guild = &interaction.guild_id.unwrap();

	let mut channels = guild.channels(&ctx).await.unwrap();

	let Some((_, channel)) = channels.iter_mut().find(|(_, channel)| {
		channel.topic.is_some() && channel.topic.as_ref().unwrap() == &interaction.user.id.to_string()
	}) else {
		anyhow::bail!("You don't have a blog silly goose!")
	};

	channel.edit(&ctx, EditChannel::new().name(name)).await.unwrap();

	interaction.create_response(&ctx, response).await?;

	Ok(())
}

pub async fn delete(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been deleted!");
	let response = CreateInteractionResponse::Message(message);

	let guild = &interaction.guild_id.unwrap();

	let channels = guild.channels(&ctx).await.unwrap();

	let Some((_, channel)) = channels.iter().find(|(_, channel)| {
		channel.topic.is_some() && channel.topic.as_ref().unwrap() == &interaction.user.id.to_string()
	}) else {
		anyhow::bail!("You don't have a blog silly goose!")
	};

	channel.delete(&ctx).await.unwrap();

	interaction.create_response(&ctx, response).await?;

	Ok(())
}
