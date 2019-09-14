use serde::{Deserialize};
use serenity::model::id::{
    UserId, RoleId
};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct StaticConfiguration {
    /// Configuration for the server
    pub server: ServerConfig,
    /// Configuration for the different roles
    pub roles: RolesConfig,
    /// Configuration for the different classes and their levels
    pub classes: HashMap<String, HashMap<String, ClassConfig>>
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    /// The server id to manage
    pub id: String,
    /// The owner of the bot
    pub owner: UserId,
    /// The adminatrator roles that can run admin commands
    pub admin_roles: Vec<RoleId>,
}

#[derive(Deserialize, Debug)]
pub struct RolesConfig {
    /// Roles to auto assign on join
    pub auto_assign: Vec<RoleId>,
    /// The role to designate that a user has joined the server
    pub joined: Vec<RoleId>
}

#[derive(Deserialize, Debug)]
pub struct ClassConfig {
    /// The role that denotes the possesion of the class
    role: RoleId,
    /// The name of the class
    name: String
}