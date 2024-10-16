use {
    super::DbResult,
    crate::{
        db::{
            models::{
                relationships::NewScheduleMusculature,
                schedule::{
                    NewSchedule,
                    Schedule,
                },
            },
            Database,
        },
        schema::{
            schedules,
            schedules_musculatures,
        },
    },
    chrono::NaiveTime,
    diesel::{
        prelude::*,
        result::Error,
    },
    diesel_async::{
        AsyncConnection,
        RunQueryDsl,
    },
    std::sync::Arc,
};

pub struct ScheduleRepository {
    pub db: Arc<Database>,
}

impl ScheduleRepository {
    pub async fn get_by_user_id(&self, user_id: &str) -> DbResult<Vec<Schedule>> {
        let mut conn = self.db.get_connection().await;
        schedules::table
            .filter(schedules::user_id.eq(user_id))
            .select(Schedule::as_select())
            .load(&mut conn)
            .await
    }

    pub async fn get_by_user_id_and_day(&self, user_id: &str, day: &str) -> DbResult<Schedule> {
        let mut conn = self.db.get_connection().await;
        schedules::table
            .filter(schedules::user_id.eq(user_id))
            .filter(schedules::day.eq(day))
            .select(Schedule::as_select())
            .first(&mut conn)
            .await
    }

    pub async fn get_by_id(&self, id: i32) -> DbResult<Schedule> {
        let mut conn = self.db.get_connection().await;
        schedules::table
            .find(id)
            .select(Schedule::as_select())
            .first(&mut conn)
            .await
    }

    // insert method
    pub async fn insert(
        &self,
        user_id: String,
        channel_id: String,
        day: String,
        start_time: NaiveTime,
    ) -> DbResult<Schedule> {
        let new_schedule = NewSchedule {
            user_id: &user_id,
            channel_id: &channel_id,
            day: &day,
            start_time: &start_time,
        };

        let mut conn = self.db.get_connection().await;
        diesel::insert_into(schedules::table)
            .values(&new_schedule)
            .get_result(&mut conn)
            .await
    }

    // update method
    pub async fn update(
        &self,
        user_id: String,
        day: String,
        start_time: Option<NaiveTime>,
    ) -> Result<Schedule, Error> {
        let mut conn = self.db.get_connection().await;
        let mut schedule = self.get_by_user_id_and_day(&user_id, &day).await?;

        if let Some(start_time) = start_time {
            schedule.start_time = start_time;
        }

        diesel::update(schedules::table.find(schedule.id))
            .set(&schedule)
            .returning(Schedule::as_returning())
            .get_result(&mut conn)
            .await
    }

    pub async fn add_musculature_to_schedule(
        &self,
        schedule_id: i32,
        musculature_id: i32,
    ) -> DbResult<()> {
        let mut conn = self.db.get_connection().await;
        let new_schedule_musculature = NewScheduleMusculature {
            schedule_id: &schedule_id,
            musculature_id: &musculature_id,
        };

        diesel::insert_into(schedules_musculatures::table)
            .values(new_schedule_musculature)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn update_musculature_in_schedule(
        &self,
        schedule_id: i32,
        musculature_ids: Vec<i32>,
    ) -> DbResult<()> {
        let mut conn = self.db.get_connection().await;

        conn.transaction(|conn| {
            Box::pin(async move {
                // Delete existing associations
                diesel::delete(schedules_musculatures::table)
                    .filter(schedules_musculatures::schedule_id.eq(&schedule_id))
                    .execute(conn)
                    .await?;

                // Insert new associations
                let new_associations: Vec<NewScheduleMusculature> = musculature_ids
                    .iter()
                    .map(|musculature_id| {
                        NewScheduleMusculature {
                            schedule_id: &schedule_id,
                            musculature_id,
                        }
                    })
                    .collect();

                diesel::insert_into(schedules_musculatures::table)
                    .values(&new_associations)
                    .execute(conn)
                    .await?;

                Ok(())
            })
        })
        .await
    }

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
