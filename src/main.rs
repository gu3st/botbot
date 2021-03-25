use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::{Message, ReactionType};
use regex::Regex;
#[macro_use] extern crate lazy_static;

use std::env;
use serenity::client::bridge::gateway::GatewayIntents;


struct Handler;



#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx:Context, msg: Message){
        lazy_static! {
            static ref MANREGEX: Regex = Regex::new(r"(?i)[^A-z]?man[^A-z]").unwrap();
        }
        if MANREGEX.is_match(msg.content.trim()){
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