use log::{debug, error, warn};
use serenity::client::Client;
use std::env;
use std::fs;

pub mod bot_data;
pub mod commands;
pub mod config;
pub mod db;
pub mod discord;
pub mod model;
pub mod status_logger;

use config::StaticConfiguration;
use discord::events::Handler;

pub fn load_environment() {
    // Load .env
    match dotenv::dotenv() {
        Ok(_) => debug!("Loaded .env"),
        Err(_) => warn!("Failed to load .env"),
    };

    // Load .env.loacal
    match dotenv::from_filename(".env.local") {
        Ok(_) => debug!("Loaded .env.local"),
        Err(e) => warn!("Failed to load .env.local: {:?}", e),
    };

    // Load logger
    pretty_env_logger::init();
    debug!("Initialized logger");
}

pub fn connect_discord() -> Client {
    match env::var("TOKEN") {
        // Login with a bot token from the environment
        Ok(token) => match Client::new(&token, Handler) {
            Ok(c) => return c,
            Err(e) => {
                let message = format!("Failed to create Client: {:?}", e);
                error!("{}", message);
                panic!("{}", message);
            }
        },
        Err(e) => {
            let message = format!("Failed to load TOKEN environment variable: {:?}", e);
            error!("{}", message);
            panic!("{}", message);
        }
    }
}

pub fn load_config() -> StaticConfiguration {
    // Open file
    match fs::read_to_string("./config.toml") {
        Ok(file) => {
            // Load config
            debug!("Loaded config");
            match toml::from_str::<StaticConfiguration>(&file) {
                Ok(c) => {
                    debug!("Parses config");
                    return c;
                }
                Err(e) => {
                    let message = format!("Error parsing config: {}", e);
                    error!("{}", message);
                    panic!("{}", message);
                }
            };
        }
        Err(e) => {
            let message = format!("Failed to load ./config.toml: {:?}", e);
            error!("{}", message);
            panic!("{}", message);
        }
    };
}
