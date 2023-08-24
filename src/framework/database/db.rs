use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

pub struct Database<'a, T> {
    db: &'a dyn Connection<Backend = T, TransactionManager = ()>,
    dsn: String,
    dsn_test: String,
    db_type: String,
    db_type_test: String,
    debug: bool,
    auto_migrate: bool,
    env: String,
}

impl<'a, T> Database<'a, T> {
    pub fn new(
        db: &'a dyn Connection<Backend = T, TransactionManager = ()>,
        dsn: String,
        dsn_test: String,
        db_type: String,
        db_type_test: String,
        debug: bool,
        auto_migrate: bool,
        env: String,
    ) -> Self {
        Self {
            db,
            dsn,
            dsn_test,
            db_type,
            db_type_test,
            debug,
            auto_migrate,
            env,
        }
    }
    pub fn new_db_test(&self) -> &'a dyn Connection<Backend = T, TransactionManager = ()> {
        let db_instance = Database::new(
            self.establish_pg_connection(),
            "".to_string(),
            ":memory".to_string(),
            "".to_string(),
            "sqlite3".to_string(),
            true,
            false,
            "test".to_string(),
        );
        let connection = db_instance.db;
        connection
    }

    pub fn establish_pg_connection(
        &self,
    ) -> &'a dyn Connection<Backend = T, TransactionManager = ()> {
        dotenv().ok();
        if self.env != "production" {
            return &PgConnection::establish(&self.dsn)
                .expect(&format!("Error connecting to {}", self.db_type));
        }
        return &SqliteConnection::establish(&self.dsn_test)
            .expect(&format!("Error connecting to {}", self.db_type_test));
    }
}
