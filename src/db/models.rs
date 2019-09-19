use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serenity::model::id::{RoleId, UserId};
use serenity::model::misc::{RoleIdParseError, UserIdParseError};
use uuid::{parser::ParseError, Uuid};

#[derive(Queryable, Debug)]
pub struct DatabaseUser {
    /// The discord ID of the user
    id: String,
    /// The users display name
    pub name: String,
    /// The ids of the classes that the user is in seperated by commas
    classes: String,
}

impl DatabaseUser {
    pub fn get_classes_ids(&self) -> Result<Vec<Uuid>, ParseError> {
        self.classes
            .split(",")
            .map(|x| Uuid::parse_str(x))
            .collect()
    }

    pub fn get_classes(&self, connection: &SqliteConnection) -> QueryResult<Vec<DatabaseClass>> {
        use crate::db::schema::classes::dsl::*;

        self.classes
            .split(",")
            .map(|x| classes.find(x).first(connection))
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
    pub fn get_id(&self) -> Uuid {
        Uuid::parse_str(&self.id).expect("Malformed class id")
    }

    pub fn get_role(&self) -> Result<RoleId, RoleIdParseError> {
        self.role.parse::<RoleId>()
    }
}
