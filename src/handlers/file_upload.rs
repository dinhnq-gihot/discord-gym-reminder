use anyhow::Result;
use chrono::{Datelike, Utc};
use serenity::{
    all::{Context, Http, Message, MessageBuilder},
    model::user,
};
use std::{
    fs::File,
    io::{Read, Write},
    sync::Arc,
};

use crate::db::{
    models::Exercise,
    repositories::{ExerciseRepository, MusculatureRepository, ScheduleRepository},
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
    let channel_id = msg.channel_id.to_string();

    for schedule in schedules.into_iter() {
        let a = repo
            .insert(
                user_id.clone(),
                channel_id.clone(),
                schedule.day,
                parse_start_time(&schedule.start_time).unwrap_or_default(),
                schedule.musculatures,
            )
            .await?;

        println!("{a:#?}");
    }

    Ok(())
}

pub async fn get_all_exercises_in_schedule(
    db: Arc<Database>,
    musculatures: Vec<String>,
) -> Result<Vec<Exercise>> {
    let mus_repo = MusculatureRepository {
        db: Arc::clone(&db),
    };
    let exercise_repo = ExerciseRepository {
        db: Arc::clone(&db),
    };
    let mut ret = Vec::new();
    for m in musculatures.into_iter() {
        let mus = mus_repo.get_by_name(&m).await?;
        let mut exers = exercise_repo.get_by_musculature(mus.id).await?;
        ret.append(&mut exers);
    }

    Ok(ret)
}

pub async fn reminder(http: Arc<Http>, db: Arc<Database>) -> Result<()> {
    println!("Reminder scanning...");
    let schedule_repo = ScheduleRepository {
        db: Arc::clone(&db),
    };

    let now = Utc::now().time();
    let current_weekday = Utc::now().naive_utc().date().weekday().to_string();

    let schedules = schedule_repo
        .get_upcoming_by_day_in_time(current_weekday.clone(), now)
        .await?;

    println!("Now: {now:#?}");
    println!("Current weekday: {}", &current_weekday);

    println!("{schedules:#?}");

    for s in schedules.into_iter() {
        let user_id = s.user_id;
        let channel_id = s.channel_id.parse::<u64>()?; // Parse as u64
        let channel_id = serenity::model::id::ChannelId::from(channel_id); // Convert to ChannelId
        let time = s.start_time.to_string();

        let message = MessageBuilder::new()
            .push(format!(
                "Tới giờ đi tập của thloz <@{}> lúc {}",
                user_id, time
            ))
            .build();

        let _ = channel_id.say(&http, &message).await?;

        let exers = get_all_exercises_in_schedule(
            Arc::clone(&db),
            s.musculatures
                .into_iter()
                .filter_map(|m| m)
                .collect::<Vec<String>>(),
        )
        .await?;

        for e in exers.into_iter() {
            if let Err(why) = channel_id.say(&http, e.format_for_discord()).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    Ok(())
}
