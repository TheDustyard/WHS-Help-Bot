use std::sync::{Arc, Mutex};
use crate::config::StaticConfiguration;
use diesel::sqlite::SqliteConnection;
use serenity::prelude::TypeMapKey;

pub struct SqliteDatabaseConnection;

impl TypeMapKey for SqliteDatabaseConnection {
    type Value = Arc<Mutex<SqliteConnection>>;
}

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = StaticConfiguration;
}