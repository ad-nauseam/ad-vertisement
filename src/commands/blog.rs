use std::time::Duration;

use anyhow::Result;
use serenity::all::{
	ActionRowComponent, CommandDataOption, CommandInteraction, Context, CreateActionRow, CreateChannel,
	CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, CreateWebhook,
	EditChannel, ModalInteractionCollector, PermissionOverwrite, PermissionOverwriteType, Permissions, Webhook,
};

use crate::blogs::Blogs;

pub async fn create(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let mut blogs = Blogs::new(ctx, interaction).await?;

	if blogs.channel(interaction.user.id).is_ok() {
		anyhow::bail!("You already have a blog channel");
	}

	let builder = CreateChannel::new(&interaction.user.name)
		.category(blogs.category)
		.topic(interaction.user.id.to_string())
		.permissions([
			PermissionOverwrite {
				allow: Permissions::empty(),
				deny: Permissions::SEND_MESSAGES,
				kind: PermissionOverwriteType::Role(blogs.guild.everyone_role()),
			},
			PermissionOverwrite {
				allow: Permissions::SEND_MESSAGES,
				deny: Permissions::empty(),
				kind: PermissionOverwriteType::Member(interaction.user.id),
			},
		]);

	let channel = blogs.guild.create_channel(ctx, builder).await?;

	blogs.channels.push(channel);
	blogs.reorder(ctx).await?;

	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been created!");
	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(ctx, response).await?;

	Ok(())
}

pub async fn delete(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let mut blogs = Blogs::new(ctx, interaction).await?;
	let channel = blogs.channel(interaction.user.id)?;

	let custom_id = interaction.id.to_string();

	let modal =
		CreateModal::new(&custom_id, "Blog Deletion Confirmation").components(vec![CreateActionRow::InputText(
			CreateInputText::new(
				serenity::all::InputTextStyle::Short,
				format!("Enter {} for confirmation", channel.name),
				&custom_id,
			),
		)]);

	let response = CreateInteractionResponse::Modal(modal);

	interaction.create_response(ctx, response).await?;

	let modal_interaction = ModalInteractionCollector::new(&ctx.shard)
		.timeout(Duration::new(60, 0))
		.custom_ids(vec![custom_id])
		.await
		.unwrap();

	let ActionRowComponent::InputText(text) = &modal_interaction.data.components[0].components[0] else {
		anyhow::bail!("Expected text input component")
	};

	let Some(value) = &text.value else {
		anyhow::bail!("Expected value in text input")
	};

	let content = if value.trim() == &channel.name {
		channel.delete(&ctx).await?;
		"Your blog channel has been deleted!"
	} else {
		"Blog deletion cancelled, please enter correct name."
	};

	let message = CreateInteractionResponseMessage::new().content(content);
	let response = CreateInteractionResponse::Message(message);

	modal_interaction.create_response(ctx, response).await?;

	Ok(())
}

pub async fn rename(ctx: &Context, interaction: &CommandInteraction, options: &[CommandDataOption]) -> Result<()> {
	let mut blogs = Blogs::new(ctx, interaction).await?;
	let channel = blogs.channel(interaction.user.id)?;

	let name = match options.first().and_then(|option| option.value.as_str()) {
		Some(name) => name,
		None => &interaction.user.name,
	};

	channel.edit(ctx, EditChannel::new().name(name)).await?;
	blogs.reorder(ctx).await?;

	let message = CreateInteractionResponseMessage::new().content("Your blog channel has been renamed!");
	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(ctx, response).await?;

	Ok(())
}

pub async fn webhook(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
	let mut blogs = Blogs::new(ctx, interaction).await?;
	let channel = blogs.channel(interaction.user.id)?;

	let webhooks = channel.webhooks(ctx).await?;

	let url = match webhooks.iter().flat_map(Webhook::url).next() {
		Some(url) => url,
		None => channel.create_webhook(ctx, CreateWebhook::new("Blog")).await?.url()?,
	};

	let message = CreateInteractionResponseMessage::new()
		.content(format!("Creation of your webhook was successful!\n\n-# {url}"))
		.ephemeral(true);

	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(ctx, response).await?;

	Ok(())
}
