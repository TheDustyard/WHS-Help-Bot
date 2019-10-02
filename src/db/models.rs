use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sqlite::SqliteConnection;
use serenity::model::id::{RoleId, UserId};
use serenity::model::misc::{RoleIdParseError, UserIdParseError};
use std::io::Write;
use uuid::{parser::ParseError, Uuid};

#[derive(Queryable, Debug)]
pub struct DatabaseUser {
    /// The discord ID of the user
    pub id: String,
    /// The users display name
    pub name: String,
    /// The ids of the classes that the user is in seperated by commas
    pub classes: String,
}

impl DatabaseUser {
    pub fn get_classes_ids(&self) -> Result<Vec<Uuid>, ParseError> {
        self.classes
            .split(",")
            .map(|x| Uuid::parse_str(x))
            .collect()
    }

    pub fn get_classes(&self, connection: &SqliteConnection) -> Vec<Result<DatabaseClass, &str>> {
        use crate::db::schema::classes::dsl::*;

        self.classes
            .split(",")
            .filter(|x| x.len() > 0)
            .map(
                |x| match classes.find(x).first::<DatabaseClass>(connection) {
                    Ok(class) => Ok(class),
                    Err(_) => Err(x),
                },
            )
            .collect()
    }

    pub fn get_id(&self) -> Result<UserId, UserIdParseError> {
        self.id.parse::<UserId>()
    }
}

#[derive(Queryable, Debug)]
pub struct DatabaseClass {
    /// The uuid of the class
    id: String,
    /// The class name
    pub name: String,
    /// The role to use to display the class
    role: String,
}

impl DatabaseClass {
    /// Parse the id of the class
    ///
    /// ## Panics
    /// If the uuid is malformed
    /// (Should never happen unless a fatal
    /// user error or breaking change in UUID lib)
    pub fn get_id(&self) -> Uuid {
        Uuid::parse_str(&self.id).expect("Attempted to get a malformatted uuid")
    }

    /// Parse the role of the class
    /// 
    /// ## Panics
    pub fn get_role(&self) -> Result<RoleId, RoleIdParseError> {
        self.role.parse::<RoleId>()
    }
}
