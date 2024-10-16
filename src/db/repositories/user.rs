use {
    super::DbResult,
    crate::{
        db::{
            models::user::{
                NewUser,
                User,
            },
            Database,
        },
        schema::users,
    },
    diesel::{
        update,
        QueryDsl,
        SelectableHelper,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
};

pub struct UserRepository {
    pub db: Arc<Database>,
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

    pub async fn get_all(&self) -> DbResult<Vec<User>> {
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
        let mut user: User = users::table
            .find(&user_id)
            .select(User::as_select())
            .first(&mut conn)
            .await?;

        if let Some(name) = name {
            user.name = Some(name);
        }
        if let Some(point) = point {
            user.point = point;
        }
        if let Some(level) = level {
            user.level = level;
        }

        update(users::table.find(user_id))
            .set(user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
    }
}
