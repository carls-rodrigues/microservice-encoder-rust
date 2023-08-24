use dotenvy::dotenv;
use sqlx::{Connection, PgPool};

// todo: impl interfaces for other databases
pub struct Database {
    database_url: String,
    debug: bool,
    env: String,
}

impl Database {
    pub fn new(database_url: String, debug: bool, env: String) -> Self {
        Self {
            database_url,
            debug,
            env,
        }
    }
    pub async fn new_db_test() -> PgPool {
        let db = Database::new(
            "postgres://postgres:root@localhost:5432/encoder?sslmode=disable".to_string(),
            true,
            "test".to_string(),
        );
        let connection = Database::establish_pg_connection(&db).await;
        connection
    }

    pub async fn establish_pg_connection(&self) -> PgPool {
        dotenv().ok();
        return PgPool::connect(&self.database_url)
            .await
            .expect(&format!("Error connecting to {}", self.database_url));
    }
}
