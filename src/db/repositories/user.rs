use super::DbResult;
use crate::{
    db::{
        models::user::{NewUser, User},
        Database,
    },
    schema::users,
};
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub async fn create(&self, new_user_id: String, name: Option<String>) -> DbResult<User> {
        let mut conn = self.db.get_connection().await;
        let new_user = NewUser {
            id: &new_user_id,
            name: name.as_deref(),
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
    }

    pub async fn get_by_id(&self, user_id: String) -> DbResult<User> {
        let mut conn = self.db.get_connection().await;
        users::table.find(user_id).first(&mut conn).await
    }

    pub async fn find_all(&self) -> DbResult<Vec<User>> {
        let mut conn = self.db.get_connection().await;
        users::table.load::<User>(&mut conn).await
    }

    pub async fn update(
        &self,
        user_id: String,
        name: Option<String>,
        point: Option<i32>,
        level: Option<i32>,
    ) -> DbResult<User> {
        let mut conn = self.db.get_connection().await;
        let user = users::table.find(user_id).first(&mut conn).await?;

        diesel::update(users::table.find(&user.id))
            .set(user)
            .get_result(&mut conn)
    }

    pub fn delete(pool: &DbPool, user_id: &str) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::delete(users::table.find(user_id)).execute(&mut conn)
    }
}
