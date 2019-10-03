//! The TypeMapKeys that are stored in the bot data

use crate::config::StaticConfiguration;
use diesel::sqlite::SqliteConnection;
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};

pub struct SqliteDatabaseConnection;

impl TypeMapKey for SqliteDatabaseConnection {
    type Value = Arc<Mutex<SqliteConnection>>;
}

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = StaticConfiguration;
}
