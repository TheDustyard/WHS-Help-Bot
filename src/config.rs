use serde::Deserialize;
use serenity::model::id::{GuildId, RoleId, UserId};

#[derive(Deserialize, Debug)]
pub struct StaticConfiguration {
    /// Configuration for the bot
    pub bot: BotConfig,
    /// Configuration for the server
    pub server: ServerConfig,
    /// Configuration for the different roles
    pub roles: RolesConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    /// The server id to manage
    pub id: GuildId,
    /// The adminatrator roles that can run admin commands
    pub admin_roles: Vec<RoleId>,
}

#[derive(Deserialize, Debug)]
pub struct RolesConfig {
    /// Roles to auto assign on join
    pub auto_assign: Vec<RoleId>,
    /// The role to designate that a user has joined the server
    pub joined: RoleId,
}

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    /// The prefix for the bot
    pub prefix: String,
    /// The owner of the bot
    pub owner: UserId,
}