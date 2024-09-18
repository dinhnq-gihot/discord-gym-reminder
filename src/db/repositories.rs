use std::sync::Arc;

use crate::db::models::{Exercise, Musculature};
use crate::schema::{exercises, musculature};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

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
}
