pub mod models;
pub mod repositories;

use {
    diesel_async::{
        pooled_connection::{
            bb8::{
                Pool,
                PooledConnection,
            },
            AsyncDieselConnectionManager,
        },
        AsyncPgConnection,
    },
    diesel_async_migrations::{
        embed_migrations,
        EmbeddedMigrations,
    },
    once_cell::sync::Lazy,
    std::fmt::Debug,
};

pub static MIGRATIONS: Lazy<EmbeddedMigrations> = Lazy::new(|| embed_migrations!("migrations"));

#[derive(Clone)]
pub struct Database {
    pool: Pool<AsyncPgConnection>,
    _url: String,
    _in_use: bool,
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("url", &self._url)
            .field("_in_use", &self._in_use)
            .finish()
    }
}

impl Database {
    pub async fn new(url: &str) -> Self {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .await
            .expect("Could not build connection pool");
        let mut _conn = pool.get_owned().await.unwrap();
        MIGRATIONS.run_pending_migrations(&mut _conn).await.unwrap();

        Database {
            pool,
            _url: url.to_string(),
            _in_use: true,
        }
    }

    pub async fn get_connection(&self) -> PooledConnection<AsyncPgConnection> {
        self.pool.get().await.unwrap()
    }
}
