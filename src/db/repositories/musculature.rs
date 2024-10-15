use crate::db::models::musculature::Musculature;
use crate::db::Database;
use crate::schema::musculatures;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub struct MusculatureRepository {
    pub db: Arc<Database>,
}

impl MusculatureRepository {
    pub async fn get_all(&self) -> Result<Vec<Musculature>, Error> {
        let mut conn = self.db.get_connection().await;
        musculatures::table.load::<Musculature>(&mut conn).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Musculature, Error> {
        let mut conn = self.db.get_connection().await;
        musculatures::table.find(id).get_result(&mut conn).await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Musculature, Error> {
        let mut conn = self.db.get_connection().await;
        musculatures::table
            .filter(musculatures::name.eq(name))
            .get_result(&mut conn)
            .await
    }
}
