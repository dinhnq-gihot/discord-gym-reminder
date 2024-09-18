mod db;
mod handlers;
mod schema;

use dotenv::dotenv;
use handlers::Handler;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_BOT_TOKEN").expect("Cannot get discord bot token!");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error createing client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
