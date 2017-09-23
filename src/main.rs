extern crate serenity;
extern crate time;
extern crate tzbot;

use serenity::CACHE;
use serenity::prelude::*;
use serenity::model::*;
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

impl EventHandler for Handler {
    fn on_message(&self, _: Context, msg: Message) {
        // don't talk to yourself, bot
        if msg.author.id == CACHE.read().unwrap().user.id {
            log!("SELF:[{}] | chan:{}", msg.content, msg.channel_id);
            return;
        }

        if msg.content == "!tzbot" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                log!("Error sending message: {:?}", why);
            }
        }

        let result = convert(&msg.content);
        match result {
            None => {
                log!("NOPE:[{}] user:[{}]", msg.content, msg.author.name);
            },
            Some(val) => {
                if let Err(why) = msg.channel_id.say(val) {
                    log!("Error sending message: {:?}", why);
                }
                log!("OK  :[{}] | user:[{}] | chan:{}", msg.content, msg.author.name, msg.channel_id);
            },
        }
    }

    // In this case, just print what the current user's username is.
    fn on_ready(&self, _: Context, ready: Ready) {
        log!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler);

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        log!("Client error: {:?}", why);
    }
}
