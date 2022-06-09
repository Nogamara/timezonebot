extern crate serenity;
extern crate time;
extern crate tzbot;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use tzbot::convert;

struct Handler;

macro_rules! log {
    () => (
        println!("{}", now());
    );
    ($x:expr) => (
        print!("{} ", now());
        println!($x);
    );
    ($x:expr, $($arg:tt)*) => (
        print!("{} ", now());
        println!($x, $($arg)*);
    );
}

fn now() -> String {
    let now = time::now();

    match time::strftime("[%Y-%m-%d %H:%M:%S]", &now) {
        Err(_) => "[XXXX-XX-XX xx:xx:xx]".to_string(),
        Ok(s) => s,
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // don't talk to yourself, bot
        if msg.is_own(&context.cache) {
            log!("SELF:[{}] | chan:{}", msg.content, msg.channel_id);
            return;
        }

        if msg.content == "!tzbot" {
            if let Err(why) = msg.channel_id.say(&context.http, "Pong!").await {
                log!("Error sending message: {:?}", why);
            }
        }

        let result = convert(&msg.content);
        match result {
            None => {
                log!("NOPE:[{}] user:[{}]", msg.content, msg.author.name);
            }
            Some(val) => {
                if let Err(why) = msg.channel_id.say(&context.http, val).await {
                    log!("Error sending message: {:?}", why);
                }
                log!(
                    "OK  :[{}] | user:[{}] | chan:{}",
                    msg.content,
                    msg.author.name,
                    msg.channel_id
                );
            }
        }
    }

    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        log!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES;
    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        log!("Client error: {:?}", why);
    }
}
