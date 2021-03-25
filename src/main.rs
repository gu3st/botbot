use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::{Message, ReactionType};
use serenity::model::id::{EmojiId};

use std::env;
use serenity::client::bridge::gateway::GatewayIntents;


struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx:Context, msg: Message){
        if msg.content.trim() == "man" {
            let emote = ctx.http.get_emoji(788585865060155392, 791766387143081994).await.expect("Error fetching emote");
            let reaction = ReactionType::Custom {
                animated: false,
                id: emote.id,
                name: Some(emote.name)
            };
            if let Err(why) = msg.react(ctx.http, reaction).await{
                println!("An error occurred while reacting: {:?}", why)
            }
        }
    }

}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .intents(GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_EMOJIS)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}