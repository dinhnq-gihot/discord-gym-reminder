use crate::db::models::schedule::{NewSchedule, Schedule};
use crate::db::Database;
use crate::schema::schedules;
use chrono::NaiveTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub struct ScheduleRepository {
    pub db: Arc<Database>,
}

impl ScheduleRepository {
    pub async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<Schedule>, Error> {
        let mut conn = self.db.get_connection().await;
        schedules::table
            .filter(schedules::user_id.eq(user_id))
            .select(Schedule::as_select())
            .load(&mut conn)
            .await
    }

    pub async fn get_by_user_id_and_day(
        &self,
        user_id: &str,
        day: &str,
    ) -> Result<Schedule, Error> {
        let mut conn = self.db.get_connection().await;
        schedules::table
            .filter(schedules::user_id.eq(user_id))
            .filter(schedules::day.eq(day))
            .select(Schedule::as_select())
            .first(&mut conn)
            .await
    }

    // insert method
    // pub async fn insert(
    //     &self,
    //     user_id: String,
    //     channel_id: String,
    //     day: String,
    //     start_time: NaiveTime,
    //     musculatures: Vec<String>,
    // ) -> Result<Schedule, Error> {
    //     let new_schedule = NewSchedule {
    //         user_id: &user_id,
    //         channel_id: &channel_id,
    //         day: &day,
    //         start_time: &start_time,
    //         musculatures: &musculatures,
    //     };

    //     let mut conn = self.db.get_connection().await;
    //     diesel::insert_into(schedules::table)
    //         .values(&new_schedule)
    //         .get_result(&mut conn)
    //         .await
    // }

    // update method
    // pub async fn update(
    //     &self,
    //     user_id: String,
    //     day: String,
    //     start_time: Option<NaiveTime>,
    //     musculatures: Option<Vec<String>>,
    //     reminded: Option<bool>,
    // ) -> Result<Schedule, Error> {
    //     let mut conn = self.db.get_connection().await;
    //     let mut schedule = self.get_by_user_id_and_day(&user_id, &day).await?;

    //     if let Some(start_time) = start_time {
    //         schedule.start_time = start_time;
    //     }
    //     if let Some(musculatures) = musculatures {
    //         schedule.musculatures = musculatures.into_iter().map(Some).collect();
    //     }
    //     if let Some(reminded) = reminded {
    //         schedule.reminded = Some(reminded);
    //     }

    //     diesel::update(schedules::table.find(schedule.id))
    //         .set(&schedule)
    //         .returning(Schedule::as_returning())
    //         .get_result(&mut conn)
    //         .await
    // }

    pub async fn get_upcoming_by_day_in_time(
        &self,
        day: String,
        time: NaiveTime,
    ) -> Result<Vec<Schedule>, Error> {
        let mut conn = self.db.get_connection().await;
        schedules::table
            .filter(schedules::day.eq(day))
            .filter(schedules::start_time.gt(time))
            .select(Schedule::as_select())
            .load(&mut conn)
            .await
    }

    // delete method
    pub async fn delete(&self, user_id: &str, day: &str) -> Result<usize, Error> {
        let mut conn = self.db.get_connection().await;
        diesel::delete(schedules::table)
            .filter(schedules::user_id.eq(user_id))
            .filter(schedules::day.eq(day))
            .execute(&mut conn)
            .await
    }
}