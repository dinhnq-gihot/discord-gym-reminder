pub mod file_upload;
pub mod types;
mod utils;

use std::sync::Arc;

use file_upload::handler_schedule;
use serenity::all::MessageBuilder;
use serenity::async_trait;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::db::Database;

pub struct Handler {
    pub db: Arc<Database>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");

                    return;
                }
            };
            let user_id = msg.author.id;
            println!("User ID: {}", user_id);

            let response = MessageBuilder::new()
                .push("User ")
                .mention(&user_id)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "!schedule" {
            if !msg.attachments.is_empty() {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Upload schedule sucessfully!")
                    .await
                {
                    println!("Error sending message: {why:?}");
                }

                if let Err(why) = handler_schedule(&ctx, &msg, &self.db).await {
                    println!("Error handle schedule: {why:?}");
                }
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Missing schedule file, please import this!")
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }

    // async fn command(&self, ctx: Context, interaction: Interaction) {
    //     if let Interaction::Command(command) = interaction {
    //         if let Some(cmd) = Command::from(value)
    //     }
    // }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
