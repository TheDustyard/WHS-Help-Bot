#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use prettytable::{cell, row, Table};
use serenity::client::Client;
use serenity::model::{
    gateway::{Activity, Ready},
    user::OnlineStatus,
};
use serenity::prelude::*;
use std::env;
use std::fs;
use std::io::Write;

use crate::db::models::{DatabaseClass, DatabaseUser};

pub mod bot_data;
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

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_presence(Some(Activity::listening("!help")), OnlineStatus::Online);
    }
}

pub fn connect_discord() -> Client {
    let token = env::var("TOKEN").expect("Missing TOKEN environment variable");
    // Login with a bot token from the environment
    Client::new(&token, Handler).expect("Error creating client")
}

#[deprecated] // FIXME:
pub fn sample_users(connection: &SqliteConnection) -> String {
    use db::schema::users::dsl::*;

    let mut w: Vec<u8> = Vec::new();

    let results = users
        .limit(5)
        .load::<DatabaseUser>(connection)
        .expect("Error loading users");

    writeln!(
        w,
        "Displaying {} of {} users",
        results.len(),
        users.count().get_result::<i64>(connection).unwrap()
    )
    .unwrap();

    let mut table = Table::new();

    table.add_row(row!["USER ID", "NAME", "CLASSES"]);

    for user in results {
        table.add_row(row![
            format!("{:?}", user.parse_id()),
            user.name,
            format!("{:?}", table_classes(user.parse_classes(connection)))
        ]);
    }

    writeln!(w, "{}", table).unwrap();

    return String::from_utf8_lossy(&w).into_owned();
}

#[deprecated] // FIXME:
pub fn sample_classes(connection: &SqliteConnection) -> String {
    use db::schema::classes::dsl::*;

    let mut w: Vec<u8> = Vec::new();

    let results = classes
        .limit(5)
        .load::<DatabaseClass>(connection)
        .expect("Error loading users");

    writeln!(
        w,
        "Displaying {} of {} classes",
        results.len(),
        classes.count().get_result::<i64>(connection).unwrap()
    )
    .unwrap();
    writeln!(w, "{}", table_classes(results)).unwrap();

    return String::from_utf8_lossy(&w).into_owned();
}

fn table_classes(classes: Vec<DatabaseClass>) -> prettytable::Table {
    let mut table = Table::new();
    table.add_row(row!["ID", "NAME"]);

    for class in classes {
        table.add_row(row![class.parse_role_id(), class.name]);
    }

    table
}

pub fn load_config() -> Result<config::StaticConfiguration, toml::de::Error> {
    // Open file
    let config_file = fs::read_to_string("./config.toml").expect("Failed to open ./config.json");
    // Load config
    toml::from_str(&config_file)
}