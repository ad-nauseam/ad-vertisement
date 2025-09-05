use std::time::Duration;

use anyhow::Result;
use serenity::all::{
	ActionRowComponent, CommandDataOption, CommandInteraction, Context, CreateActionRow, CreateChannel,
	CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, CreateWebhook,
	EditChannel, InputTextStyle, ModalInteractionCollector, PermissionOverwrite, PermissionOverwriteType, Permissions,
	Webhook,
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

	let label = format!("Enter your blog name: {}", channel.name);
	let row = CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, label, ""));

	let modal = CreateModal::new("", "Confirm Deletion").components(vec![row]);
	let response = CreateInteractionResponse::Modal(modal);

	interaction.create_response(ctx, response).await?;

	let interaction = ModalInteractionCollector::new(&ctx.shard)
		.author_id(interaction.user.id)
		.timeout(Duration::from_secs(60))
		.await
		.ok_or_else(|| anyhow::anyhow!("Modal timeout exceeded"))?;

	let Some(row) = interaction.data.components.first() else {
		anyhow::bail!("No action row present");
	};

	let Some(ActionRowComponent::InputText(input)) = row.components.first() else {
		anyhow::bail!("No input text present");
	};

	let content = if input.value.as_ref() == Some(&channel.name) {
		channel.delete(&ctx).await?;
		"Your blog channel has been deleted!"
	} else {
		"Blog deletion cancelled, please enter the correct name!"
	};

	let message = CreateInteractionResponseMessage::new().content(content);
	let response = CreateInteractionResponse::Message(message);

	interaction.create_response(ctx, response).await?;

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
