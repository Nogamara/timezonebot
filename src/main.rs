extern crate serenity;
extern crate tzbot;

use serenity::prelude::*;
use serenity::model::*;
use std::env;
use tzbot::convert;
use serenity::CACHE;

struct Handler;

impl EventHandler for Handler {
    fn on_message(&self, _: Context, msg: Message) {
        // don't talk to yourself, bot
        if msg.author.id == CACHE.read().unwrap().user.id {
            println!("talk::self - caught [{}]", msg.content);
            return;
        }

        if msg.content == "!tzbot" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }

        let result = convert(msg.content);
        if result.len() > 0 {
            if let Err(why) = msg.channel_id.say(result) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // In this case, just print what the current user's username is.
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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
        println!("Client error: {:?}", why);
    }
}
