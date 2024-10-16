mod db;
mod handlers;
mod schema;

use {
    dotenv::dotenv,
    handlers::{
        // file_upload::reminder,
        Handler,
    },
    serenity::prelude::*,
    std::{
        sync::Arc,
        time::Duration,
    },
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = std::env::var("DISCORD_BOT_TOKEN").expect("Cannot get discord bot token!");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let db = Arc::new(
        db::Database::new(
            std::env::var("DATABASE_URL")
                .expect("Cannot get database url!")
                .as_str(),
        )
        .await,
    );

    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            db: Arc::clone(&db),
        })
        .await
        .expect("Error creating client");

    let ctx_clone = Arc::clone(&client.http);

    tokio::select! {
        _ = client.start() => {},
        // _ = async move {
        //     loop {
        //         tokio::time::sleep(Duration::from_secs(10)).await;
        //         let _ = reminder(Arc::clone(&ctx_clone), Arc::clone(&db)).await;
        //     }
        // } => {}
    }
}
