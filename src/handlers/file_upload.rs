use anyhow::Result;
use serenity::all::{Context, Message};
use std::{
    fs::File,
    io::{Read, Write},
};

use super::types::Schedule;

// New function to handle file uploads
pub async fn handle_file_upload(ctx: &Context, msg: &Message) -> Result<()> {
    for attachment in &msg.attachments {
        let file_url = &attachment.url;
        let file_name = &attachment.filename;

        // Download the YAML file
        download_file(file_url, &format!("assets/uploads/{file_name}")).await?;

        // Deserialize the YAML file
        let data = read_yaml_file(file_name).await?;

        println!("{data:#?}");

        // Acknowledge the upload and print the deserialized data
        msg.channel_id
            .say(
                &ctx.http,
                format!("Uploaded and parsed file: {}", file_name),
            )
            .await?;
        println!("{:?}", data);
    }
    Ok(())
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
