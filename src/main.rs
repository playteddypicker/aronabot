use dotenv::dotenv;
use env_logger::init;
use serenity::prelude::*;

use std::{env, error::Error};

mod event_handler;
mod events;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    init();

    let token = env::var("DISCORD_TOKEN").expect("Couldn't find token.");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .event_handler(event_handler::DiscordEventHandler)
        .await
        .expect("an error occured while creating client.");

    client.start().await?;

    Ok(())
}
