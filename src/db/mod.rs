use log::{error, debug};
use rusqlite::{Connection, NO_PARAMS, Result as SQLResult};
use std::env;

pub mod model;

pub trait Migrateable {
    fn migrate_up(conn: &Connection) -> SQLResult<()>;
    fn migrate_down(conn: &Connection) -> SQLResult<()>;
}

pub trait Queryable {
}

/// A convinience struct to provide methods that can run across all of the tables
pub struct AllTables;
impl Migrateable for AllTables {
    fn migrate_up(conn: &Connection) -> SQLResult<()> {
        model::Category::migrate_up(conn)?;
        model::Class::migrate_up(conn)?;

        Ok(())
    }

    fn migrate_down(conn: &Connection) -> SQLResult<()> {
        model::Category::migrate_down(conn)?;
        model::Class::migrate_down(conn)?;

        Ok(())
    }
}

pub fn establish_connection() -> Connection {
    match env::var("DATABASE_URL") {
        Ok(database_url) => match Connection::open(&database_url) {
            Ok(conn) => {
                debug!("Connected to database: {}", database_url);

                // ENABLE FOREIGN KEYS
                conn.execute("PRAGMA foreign_keys = ON;", NO_PARAMS).unwrap();
                debug!("Enabled Foreign Keys on database: {}", database_url);

                return conn;
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
