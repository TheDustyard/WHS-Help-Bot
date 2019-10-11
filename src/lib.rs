#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serenity::client::Client;
use serenity::model::{
    gateway::{Activity, Ready},
    user::OnlineStatus,
};
use serenity::prelude::*;
use std::env;
use std::fs;

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

pub fn load_config() -> Result<config::StaticConfiguration, toml::de::Error> {
    // Open file
    let config_file = fs::read_to_string("./config.toml").expect("Failed to open ./config.json");
    // Load config
    toml::from_str(&config_file)
}