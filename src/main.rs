use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::{Message, ReactionType};
use chrono::{FixedOffset, Utc};
use fancy_regex::Regex;
use lazy_static::lazy_static;

use std::env;
use serenity::client::bridge::gateway::GatewayIntents;


struct Handler;



#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx:Context, msg: Message){
        lazy_static! {
            static ref MANREGEX: Regex = Regex::new(r"(?i)\bman\b").unwrap();
            static ref PERHAPSREGEX: Regex = Regex::new(r"(?i)\bperhaps\b").unwrap();
            static ref OOTREGEX: Regex = Regex::new(r"(?i)\bout of touch\b").unwrap();
        }
        if MANREGEX.is_match(msg.content.trim()).unwrap(){
            let emote = ctx.http.get_emoji(86542971465396224, 824895253348876298).await.expect("Error fetching emote");
            let reaction = ReactionType::Custom {
                animated: false,
                id: emote.id,
                name: Some(emote.name)
            };
            if let Err(why) = msg.react(&ctx.http, reaction).await{
                println!("An error occurred while reacting: {:?}", why)
            }
        }

        if PERHAPSREGEX.is_match(msg.content.trim()).unwrap(){
            let emote = ctx.http.get_emoji(86542971465396224, 824895866560446474).await.expect("Error fetching emote");
            let reaction = ReactionType::Custom {
                animated: false,
                id: emote.id,
                name: Some(emote.name)
            };
            if let Err(why) = msg.react(&ctx.http, reaction).await{
                println!("An error occurred while reacting: {:?}", why)
            }
        }
        if OOTREGEX.is_match(msg.content.trim()).unwrap() && Utc::now().with_timezone(&FixedOffset::west(5*3600)).format("%a") == "Thu" {
            if let Err(why) = msg.reply(&ctx.http, "https://tenor.com/bq8xu.gif").await{
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