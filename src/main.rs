use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::{Message, ReactionType};
use chrono::{FixedOffset, Utc};
use fancy_regex::Regex;
use lazy_static::lazy_static;

use std::env;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::model::guild::Emoji;

const EMOTE_SERVER:u64 = 86542971465396224;

struct Handler;

impl Handler {
    fn reaction(&self, emote: &Emoji) -> ReactionType {
        return ReactionType::Custom {
            animated: false,
            id: emote.id,
            name: Some(String::from(&emote.name))
        };
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx:Context, msg: Message){
        lazy_static! {

            static ref MANREGEX: Regex = Regex::new(r"(?i)\bman\b").unwrap();
            static ref PERHAPSREGEX: Regex = Regex::new(r"(?i)\bperhaps\b").unwrap();
            static ref OOTREGEX: Regex = Regex::new(r"(?i)\bout of touch\b").unwrap();
            static ref GARFIELDREGEX: Regex = Regex::new(r"(?i)\bgarfield\b").unwrap();
        }
        if MANREGEX.is_match(msg.content.trim()).unwrap(){
            let emote = ctx.http.get_emoji(EMOTE_SERVER, 824895253348876298).await.expect("Error fetching emote");
            if let Err(why) = msg.react(&ctx.http, self.reaction(&emote)).await{
                println!("An error occurred while reacting: {:?}", why)
            }
        }

        if PERHAPSREGEX.is_match(msg.content.trim()).unwrap(){
            let emote = ctx.http.get_emoji(EMOTE_SERVER, 824895866560446474).await.expect("Error fetching emote");
            if let Err(why) = msg.react(&ctx.http, self.reaction(&emote)).await{
                println!("An error occurred while reacting: {:?}", why)
            }
        }
        if OOTREGEX.is_match(msg.content.trim()).unwrap() && Utc::now().with_timezone(&FixedOffset::west(5*3600)).format("%a").to_string() == "Thu" {
            if let Err(why) = msg.reply(&ctx.http, "https://www.youtube.com/watch?v=Q8hp2IkI2es").await{
                println!("An error occurred while reacting: {:?}", why)
            }
        }
        if GARFIELDREGEX.is_match(msg.content.trim()).unwrap() {
            let emote = ctx.http.get_emoji(EMOTE_SERVER,829165930936402000).await.expect("Error fetching emote");
            if let Err(why) = msg.react(&ctx.http, self.reaction(&emote)).await{
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