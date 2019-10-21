use serenity::model::id::{ChannelId, GuildId, RoleId};

#[derive(Debug)]
pub struct Class {
    /// The internal ID to use for FKs
    pub id: u32,
    /// The name of the class, linked to the name of the role
    pub name: String,
    /// The role id to link this class to
    pub role: RoleId,
    /// The category that the class is in
    pub category: Option<Category>,
    /// The channel to link this class to
    pub channel: ChannelId,
}

#[derive(Debug)]
pub struct Category {
    /// The internal ID of the category
    pub id: u32,
    /// The name of the category, linked to the name of the channel_group
    pub name: String,
    /// The channel group to put all the channels into
    pub channel_group: ChannelId,
}