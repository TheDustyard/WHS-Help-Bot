use serenity::model::id::{RoleId};
use crate::db::schema::*;

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[table_name = "classes"]
pub struct DatabaseClass {
    /// The id of the role to use to display the class
    pub id: String,
    /// The class tag
    pub tag: String,
}

impl DatabaseClass {
    /// Parse the role of the class
    ///
    /// ## Panics
    /// If the RoleId is malformed
    /// (Should never happen)
    pub fn parse_role_id(&self) -> RoleId {
        self.id
            .parse::<RoleId>()
            .expect("Attempted to parse a malformed RoleID")
    }
}
