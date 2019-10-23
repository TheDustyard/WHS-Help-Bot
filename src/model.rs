use serenity::model::id::{ChannelId, RoleId};

#[derive(Debug)]
pub struct Class {
    /// The internal ID to use for FKs
    pub id: u32,
    /// The name of the class, linked to the name of the role
    pub name: String,
    /// The role id to link this class to
    pub role: RoleId,
    /// The category that the class is in
    pub group: Option<Group>,
    /// The channel to link this class to
    pub channel: ChannelId,
}

#[derive(Debug)]
pub struct Group {
    /// The internal ID of the group
    pub id: u32,
    /// The name of the category, linked to the name of the channel_group
    pub name: String,
    /// The channel group to put all the channels into
    pub channel_group: ChannelId,
    /// The VC for the specific channel group
    pub vc: ChannelId
}