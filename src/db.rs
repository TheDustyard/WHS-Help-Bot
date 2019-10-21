use log::{debug, error};
use rusqlite::{Connection, Result as SQLResult, NO_PARAMS};
use std::env;
use std::path::Path;

static CATEGORY_SQL: &[u8] = include_bytes!("sql/category.sql");
static CLASS_SQL: &[u8] = include_bytes!("sql/class.sql");

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(file: P) -> Database {
        match env::var("DATABASE_URL") {
            Ok(database_url) => match Connection::open(&database_url) {
                Ok(connection) => {
                    debug!("Connected to database: {}", database_url);

                    // ENABLE FOREIGN KEYS
                    connection
                        .execute("PRAGMA foreign_keys = ON;", NO_PARAMS)
                        .unwrap();
                    debug!("Enabled Foreign Keys on database: {}", database_url);

                    // CREATE TABLES IF NOT EXIST
                    connection
                        .execute(&String::from_utf8_lossy(CATEGORY_SQL), NO_PARAMS)
                        .unwrap();
                    connection
                        .execute(&String::from_utf8_lossy(CLASS_SQL), NO_PARAMS)
                        .unwrap();

                    return Database { connection };
                }
                Err(e) => {
                    let message = format!("Error connecting to database {}: {:?}", database_url, e);
                    error!("{}", message);
                    panic!("{}", message);
                }
            },
            Err(e) => {
                let message = format!("Failed to load DATABASE_URL environment variable: {:?}", e);
                error!("{}", message);
                panic!("{}", message);
            }
        };
    }
}

impl Display for Database {
    
}