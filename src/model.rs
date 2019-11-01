use serenity::model::id::{ChannelId, RoleId};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Class {
    /// The internal ID to use for FKs
    pub id: i64,
    /// The name of the class, linked to the name of the role
    pub name: String,
    /// The role id to link this class to
    pub role: RoleId,
    /// The category that the class is in
    pub group: Option<Group>,
    /// The channel to link this class to
    pub channel: ChannelId,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Class {}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    /// The internal ID of the group
    pub id: i64,
    /// The name of the category, linked to the name of the channel_group
    pub name: String,
    /// The channel group to put all the channels into
    pub channel_group: ChannelId,
    /// The VC for the specific channel group
    pub vc: ChannelId,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Group {}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
