use std::env;

use anyhow::Result;
use serenity::all::{
	CommandDataOptionValue, CommandInteraction, Context, CreateChannel, CreateInteractionResponse,
	CreateInteractionResponseMessage, CreateWebhook, EditChannel, GuildChannel, PermissionOverwrite,
	PermissionOverwriteType, Permissions, RoleId,
};

pub async fn create(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let Some(guild) = &interaction.guild_id else {
		anyhow::bail!("Interaction does not has a guild!")
	};

	let channels = guild.channels(&ctx).await?;

	if channels
		.iter()
		.any(|(_, channel)| channel.topic == Some(interaction.user.id.to_string()))
	{
		anyhow::bail!("Do not be greedy! Your blog channel already exists.");
	}

	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been created!");
	let response = CreateInteractionResponse::Message(message);

	let Some((category, _)) = channels.iter().find(|(_, channel)| channel.name == "Blogs") else {
		anyhow::bail!("Blogs category channel not found!")
	};

	let channel = CreateChannel::new(&interaction.user.name)
		.category(category)
		.permissions(vec![
			PermissionOverwrite {
				allow: Permissions::SEND_MESSAGES,
				deny: Permissions::empty(),
				kind: PermissionOverwriteType::Member(interaction.user.id),
			},
			PermissionOverwrite {
				allow: Permissions::empty(),
				deny: Permissions::SEND_MESSAGES,
				kind: PermissionOverwriteType::Role(RoleId::new(env::var("GUILD_ID")?.parse::<u64>()?)),
			},
		])
		.topic(interaction.user.id.to_string());

	let mut new_channel = guild.create_channel(&ctx, channel).await?;

	let channels = guild.channels(&ctx).await?;

	let mut ids = channels
		.values()
		.filter(|f| f.parent_id == Some(*category))
		.collect::<Vec<&GuildChannel>>();

	ids.sort_by(|a, b| a.name.cmp(&b.name));

	let mut last_pos = ids[0].position;
	for id in ids {
		if id.topic == Some(interaction.user.id.to_string()) {
			new_channel.edit(&ctx, EditChannel::new().position(last_pos)).await?;
			break;
		}
		last_pos = id.position
	}

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

	let Some(guild) = &interaction.guild_id else {
		anyhow::bail!("Interaction does not has a guild!")
	};

	let mut channels = guild.channels(&ctx).await?;

	let Some((_, channel)) = channels
		.iter_mut()
		.find(|(_, channel)| channel.topic == Some(interaction.user.id.to_string()))
	else {
		anyhow::bail!("You don't have a blog silly goose!")
	};

	channel.edit(&ctx, EditChannel::new().name(name)).await?;

	let channels = guild.channels(&ctx).await?;

	let Some((category, _)) = channels.iter().find(|(_, channel)| channel.name == "Blogs") else {
		anyhow::bail!("Blogs category channel not found!")
	};

	let mut ids = channels
		.values()
		.filter(|f| f.parent_id == Some(*category))
		.collect::<Vec<&GuildChannel>>();

	ids.sort_by(|a, b| a.name.cmp(&b.name));

	let mut last_pos = ids[0].position;
	for id in ids {
		if id.topic == Some(interaction.user.id.to_string()) {
			channel.edit(&ctx, EditChannel::new().position(last_pos)).await?;
			break;
		}
		last_pos = id.position
	}

	interaction.create_response(&ctx, response).await?;

	Ok(())
}

pub async fn delete(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been deleted!");
	let response = CreateInteractionResponse::Message(message);

	let Some(guild) = &interaction.guild_id else {
		anyhow::bail!("Interaction does not has a guild!")
	};

	let channels = guild.channels(&ctx).await?;

	let Some((_, channel)) = channels
		.iter()
		.find(|(_, channel)| channel.topic == Some(interaction.user.id.to_string()))
	else {
		anyhow::bail!("You don't have a blog silly goose!")
	};

	channel.delete(&ctx).await?;

	interaction.create_response(&ctx, response).await?;

	Ok(())
}

pub async fn webhook(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let webhooks = interaction.channel_id.webhooks(&ctx).await?;
	let existing_webhook = webhooks
		.iter()
		.find(|f| f.name.is_some() && f.name.as_ref().unwrap() == "BlogHook");

	let url = if existing_webhook.is_some() {
		existing_webhook.unwrap().url()?
	} else {
		let webhook = interaction
			.channel_id
			.create_webhook(&ctx, CreateWebhook::new("BlogHook"))
			.await;

		if webhook.is_err() {
			anyhow::bail!("Error while creating webhook: {}", webhook.err().unwrap())
		}

		webhook?.url()?
	};

	let message = CreateInteractionResponseMessage::new()
		.content(format!("Your blog channel's webhook URL is: {}", url))
		.ephemeral(true);

	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(&ctx, response).await?;

	Ok(())
}
