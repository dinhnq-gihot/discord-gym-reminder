use anyhow::Result;
use serenity::all::{Context, Message};
use std::{
    fs::File,
    io::{Read, Write},
    sync::Arc,
};

use crate::db::{
    repositories::{MusculatureRepository, ScheduleRepository},
    Database,
};

use super::{types::Schedule, utils::parse_start_time};

// New function to handle file uploads
async fn handle_file_upload(ctx: &Context, msg: &Message) -> Result<Vec<Schedule>> {
    for attachment in &msg.attachments {
        let file_url = &attachment.url;
        let file_name = &attachment.filename;

        // Download the YAML file
        let uuid = uuid::Uuid::new_v4().to_string();
        download_file(file_url, &format!("assets/uploads/{uuid}-{file_name}")).await?;

        // Deserialize the YAML file
        let data = read_yaml_file(file_name).await?;

        // Acknowledge the upload and print the deserialized data
        msg.channel_id
            .say(
                &ctx.http,
                format!("Uploaded and parsed file: {}", file_name),
            )
            .await?;
        return Ok(data);
    }
    Ok(vec![])
}

// Function to download the file (you need to implement this)
async fn download_file(url: &str, file_name: &str) -> Result<()> {
    // Implement the logic to download the file from the URL and save it
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_name)?;
    let content = response.bytes().await?;

    file.write_all(&content)?;

    println!("File {file_name} has been downloaded!");

    Ok(())
}

// Function to read and deserialize the YAML file
async fn read_yaml_file(file_name: &str) -> Result<Vec<Schedule>> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let schedules: Vec<Schedule> = serde_yaml::from_str(&contents)?;
    Ok(schedules)
}

pub async fn handler_schedule(ctx: &Context, msg: &Message, db: &Arc<Database>) -> Result<()> {
    let schedules = handle_file_upload(ctx, msg).await?;
    let repo = ScheduleRepository { db: Arc::clone(db) };
    let user_id = msg.author.id.to_string();

    for schedule in schedules.into_iter() {
        let a = repo
            .insert(
                user_id.clone(),
                schedule.day,
                parse_start_time(&schedule.start_time).unwrap_or_default(),
                schedule.musculatures,
            )
            .await?;

        println!("{a:#?}");
    }

    Ok(())
}
