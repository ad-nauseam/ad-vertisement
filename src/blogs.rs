use anyhow::Result;
use serenity::all::{ChannelId, CommandInteraction, Context, GuildChannel, GuildId, UserId};

pub struct Blogs {
	pub category: ChannelId,
	pub channels: Vec<GuildChannel>,
	pub guild: GuildId,
}

impl Blogs {
	pub fn channel(&mut self, user: UserId) -> Result<&mut GuildChannel> {
		self.channels
			.iter_mut()
			.find(|channel| channel.topic == Some(user.to_string()))
			.ok_or_else(|| anyhow::anyhow!("You do not have a blog"))
	}

	pub async fn new(ctx: &Context, interaction: &CommandInteraction) -> Result<Self> {
		let Some(guild) = interaction.guild_id else {
			anyhow::bail!("Interaction was not sent from a guild");
		};

		let channels = guild.channels(&ctx).await?;

		let Some((&parent, _)) = channels.iter().find(|(_, channel)| channel.name == "Blogs") else {
			anyhow::bail!("Could not find the blog category");
		};

		let children: Vec<_> = channels
			.into_values()
			.filter(|channel| channel.parent_id == Some(parent))
			.collect();

		Ok(Self {
			category: parent,
			channels: children,
			guild,
		})
	}

	pub async fn reorder(&mut self, ctx: &Context) -> Result<()> {
		self.channels.sort_by(|first, second| first.name.cmp(&second.name));

		let channels = self
			.channels
			.iter()
			.enumerate()
			.map(|(index, channel)| (channel.id, index as u64));

		self.guild.reorder_channels(ctx, channels).await?;

		Ok(())
	}
}
