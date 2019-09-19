#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serenity::client::Client;
use serenity::prelude::*;
use std::env;
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::db::models::{DatabaseClass, DatabaseUser};

pub mod commands;
pub mod config;
pub mod db;

pub fn load_environment() {
    // TODO: Iterate over env files?
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.local").ok();
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// TODO: MOVE
struct Handler;

impl EventHandler for Handler {}


// TODO: ^^
pub struct SqliteDatabaseConnection;

impl TypeMapKey for SqliteDatabaseConnection {
    type Value = Arc<Mutex<SqliteConnection>>;
}

pub fn connect_discord() -> Client {
    let token = env::var("TOKEN").expect("Missing TOKEN environment variable");
    // Login with a bot token from the environment
    Client::new(&token, Handler).expect("Error creating client")
}

#[deprecated]
pub fn sample_users<W: Write>(connection: &SqliteConnection, w: &mut W) {
    use db::schema::users::dsl::*;

    let results = users
        .limit(5)
        .load::<DatabaseUser>(connection)
        .expect("Error loading users");

    writeln!(
        w,
        "Displaying {} of {} users",
        results.len(),
        users.count().get_result::<i64>(connection).unwrap()
    ).unwrap();
    for user in results {
        writeln!(
            w,
            "{:#?}\n{:?}\n{:#?}\n\n",
            user,
            user.get_id(),
            user.get_classes(connection)
        ).unwrap();
    }
}

#[deprecated]
pub fn sample_classes<W: Write>(connection: &SqliteConnection, w: &mut W) {
    use db::schema::classes::dsl::*;

    let results = classes
        .limit(5)
        .load::<DatabaseClass>(connection)
        .expect("Error loading users");

    writeln!(
        w,
        "Displaying {} of {} classes",
        results.len(),
        classes.count().get_result::<i64>(connection).unwrap()
    ).unwrap();
    for class in results {
        writeln!(w, "{:#?}\nis valid UUID: {}\nroleId: {:?}\n\n", class, !class.get_id().is_nil(), class.get_role()).unwrap();
    }
}

pub fn load_config() -> Result<config::StaticConfiguration, toml::de::Error> {
    // Open file
    let config_file = fs::read_to_string("./config.toml").expect("Failed to open ./config.json");
    // Load config
    toml::from_str(&config_file)
}
