pub mod dto;
pub mod file_upload;
mod utils;

use {
    crate::db::Database,
    file_upload::handler_schedule,
    log::{
        debug,
        info,
        warn,
    },
    serenity::{
        all::{
            Member,
            MessageBuilder,
        },
        async_trait,
        model::{
            channel::Message,
            gateway::Ready,
        },
        prelude::*,
    },
    std::sync::Arc,
};

pub struct Handler {
    pub db: Arc<Database>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => {
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
            }
            "!schedule" => {
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
            _ => {
                // Handle other messages or do nothing
            }
        }
    }

    // async fn command(&self, ctx: Context, interaction: Interaction) {
    //     if let Interaction::Command(command) = interaction {
    //         if let Some(cmd) = Command::from(value)
    //     }
    // }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Fetch all guilds the bot is in
        let guilds = ctx.cache.guilds();

        for guild_id in guilds {
            if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
                info!("Processing guild: {}", guild.name);

                // Fetch all members of the guild
                if let Ok(members) = guild.members(&ctx.http, None, None).await {
                    let human_members: Vec<&Member> =
                        members.iter().filter(|member| !member.user.bot).collect();

                    debug!(
                        "Guild '{}' has {} human members:",
                        guild.name,
                        human_members.len()
                    );
                    for member in human_members {
                        debug!("- {} (ID: {})", member.user.name, member.user.id);
                    }
                } else {
                    warn!("Failed to fetch members for guild: {}", guild.name);
                }
            } else {
                warn!("Failed to fetch guild info for ID: {}", guild_id);
            }
        }
    }
}
