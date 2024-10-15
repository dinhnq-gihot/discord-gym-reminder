use std::sync::Arc;
use crate::db::models::exercise::Exercise;
use crate::db::Database;
use crate::schema::exercises;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

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
