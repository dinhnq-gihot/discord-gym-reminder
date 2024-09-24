use std::sync::Arc;

use crate::db::models::{Exercise, Musculature};
use crate::schema::{exercises, musculature, schedules};
use chrono::NaiveTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

use super::models::{NewSchedule, Schedule};
use super::Database;

pub struct ExerciseRepository {
    pub db: Arc<Database>,
}

impl ExerciseRepository {
    pub async fn get_all(&self) -> Result<Vec<Exercise>, Error> {
        let mut conn = self.db.get_connection().await;
        exercises::table.load(&mut conn).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Exercise, Error> {
        let mut conn = self.db.get_connection().await;
        exercises::table.find(id).get_result(&mut conn).await
    }

    pub async fn get_by_musculature(&self, musculature_id: i32) -> Result<Vec<Exercise>, Error> {
        let mut conn = self.db.get_connection().await;
        exercises::table
            .filter(exercises::musculature_id.eq(musculature_id))
            .select(Exercise::as_select())
            .load(&mut conn)
            .await
    }
}

pub struct MusculatureRepository {
    pub db: Arc<Database>,
}

impl MusculatureRepository {
    pub async fn get_all(&self) -> Result<Vec<Musculature>, Error> {
        let mut conn = self.db.get_connection().await;
        musculature::table.load::<Musculature>(&mut conn).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Musculature, Error> {
        let mut conn = self.db.get_connection().await;
        musculature::table.find(id).get_result(&mut conn).await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Musculature, Error> {
        let mut conn = self.db.get_connection().await;
        musculature::table
            .filter(musculature::name.eq(name))
            .get_result(&mut conn)
            .await
    }
}

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
    pub async fn insert(
        &self,
        user_id: String,
        day: String,
        start_time: NaiveTime,
        musculatures: Vec<String>,
    ) -> Result<Schedule, Error> {
        let new_schedule = NewSchedule {
            user_id: &user_id,
            day: &day,
            start_time: &start_time,
            musculatures: &musculatures,
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
        musculatures: Option<Vec<String>>,
    ) -> Result<Schedule, Error> {
        let mut conn = self.db.get_connection().await;
        let mut schedule = self.get_by_user_id_and_day(&user_id, &day).await?;

        if let Some(start_time) = start_time {
            schedule.start_time = start_time;
        }
        if let Some(musculatures) = musculatures {
            schedule.musculatures = musculatures.into_iter().map(Some).collect();
        }

        diesel::update(schedules::table.find(schedule.id))
            .set(&schedule)
            .returning(Schedule::as_returning())
            .get_result(&mut conn)
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

#[cfg(test)]
mod tests {
    use super::*;

    pub async fn init_db() -> Arc<Database> {
        // Implement the initialization logic for the mock database
        let db = Database::new("postgresql://gym-discord:123@localhost:25432/gym-discord").await;
        Arc::new(db)
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_exercise_repository_get_all() {
        let db = init_db().await;
        let repo = ExerciseRepository {
            db: Arc::clone(&db),
        };

        let result = repo.get_all().await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_exercise_repository_get_by_musculature() {
        let db = init_db().await;
        let repo = ExerciseRepository {
            db: Arc::clone(&db),
        };

        // Assuming you have a musculature with ID 1 for testing
        let musculature_id = 1;
        let result = repo.get_by_musculature(musculature_id).await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_musculature_repository_get_by_name() {
        let db = init_db().await;
        let repo = MusculatureRepository { db };

        let result = repo.get_by_name("NGỰC TRÊN").await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_schedule_repository_insert() {
        let db = init_db().await;
        let repo = ScheduleRepository {
            db: Arc::clone(&db),
        };

        let user_id = "user123".to_string();
        let day = "2023-10-01".to_string();
        let start_time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
        let musculatures = vec!["NGỰC TRÊN".to_string(), "ĐẨY NGỰC GIỮA".to_string()];

        let result = repo
            .insert(user_id.clone(), day.clone(), start_time, musculatures)
            .await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_schedule_repository_get_by_user_id() {
        let db = init_db().await;
        let repo = ScheduleRepository {
            db: Arc::clone(&db),
        };

        let user_id = "user123";
        let result = repo.get_by_user_id(user_id).await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_schedule_repository_update() {
        let db = init_db().await;
        let repo = ScheduleRepository {
            db: Arc::clone(&db),
        };

        let user_id = "user123".to_string();
        let day = "2023-10-01".to_string();
        let start_time = NaiveTime::from_hms_opt(10, 0, 0);
        let musculatures = Some(vec!["LƯNG XÔ".to_string()]);

        let result = repo
            .update(user_id.clone(), day.clone(), start_time, musculatures)
            .await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_schedule_repository_delete() {
        let db = init_db().await;
        let repo = ScheduleRepository {
            db: Arc::clone(&db),
        };

        let user_id = "user123";
        let day = "2023-10-01";

        let result = repo.delete(user_id, day).await;
        println!("{result:#?}");

        assert!(result.is_ok());
        // Add more assertions based on expected results
    }
}
