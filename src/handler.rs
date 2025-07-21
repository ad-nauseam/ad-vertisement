use serenity::all::{Context, EventHandler, Message, Ready};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		if message.mentions_me(&ctx).await.unwrap() {
			message.reply(&ctx, "Hello!").await.ok();
		}
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("Bot is connected! (Guilds: {})", ready.guilds.len());
	}
}
