//! The TypeMapKeys that are stored in the bot data

use crate::{
    db::Database,
    config::StaticConfiguration
};
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};

pub struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
    type Value = Arc<Mutex<Database>>;
}

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = StaticConfiguration;
}
