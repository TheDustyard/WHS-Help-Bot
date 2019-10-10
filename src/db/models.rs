use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serenity::model::id::{RoleId, UserId};
use crate::db::schema::*;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct DatabaseUser {
    /// The discord ID of the user
    pub id: String,
    /// The users display name
    pub name: String,
    /// The ids of the classes that the user is in seperated by commas
    pub classes: String,
}

impl DatabaseUser {
    /// Parse the id of the classes
    ///
    /// ## Panics
    /// If the class ids are malformed
    /// (Should never happen unless a
    /// breaking change in storage of classes)
    pub fn parse_classes_ids(&self) -> Vec<RoleId> {
        self.classes
            .split(",")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<RoleId>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Attempted to parse malformed role list")
    }

    /// Get the classes that the user is in
    ///
    /// ## Panics
    /// If the class ids are malformed
    /// (Should never happen unless a
    /// breaking change in storage of classes
    /// or a deleted class was improperly purged)
    pub fn parse_classes(&self, connection: &SqliteConnection) -> Vec<DatabaseClass> {
        use crate::db::schema::classes::dsl::*;

        self.classes
            .split(",")
            .filter(|x| x.len() > 0)
            .map(|x| classes.find(x).first::<DatabaseClass>(connection))
            .collect::<Result<Vec<_>, _>>()
            .expect("Attempted to parse malformed or orphaned class id list")
    }

    /// Sets the classes the user is in
    ///
    /// ## Panics
    /// If the class ids are malformed
    /// (Should never happen unless a
    /// breaking change in storage of classes
    /// or a deleted class was improperly purged)
    fn set_classes(&mut self, classes: Vec<DatabaseClass>) {
        self.classes = classes
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<_>>()
            .join(",");
    }

    /// Remove a class from a user
    ///
    /// ## Panics
    /// If the class ids are malformed
    /// (Should never happen unless a
    /// breaking change in storage of classes
    /// or a deleted class was improperly purged)
    pub fn remove_class(
        &mut self,
        connection: &SqliteConnection,
        class: DatabaseClass,
    ) -> Option<usize> {
        let mut classes = self.parse_classes(connection);
        let class_pos = classes
            .iter()
            .position(|x| x.parse_role_id() == class.parse_role_id())?;

        classes.swap_remove(class_pos);

        self.set_classes(classes);

        Some(class_pos)
    }

    /// Parse the id of the user
    ///
    /// ## Panics
    /// If the UserId is malformed
    /// (Should never happen)
    pub fn parse_id(&self) -> UserId {
        self.id
            .parse::<UserId>()
            .expect("Attempted to parse a malformed UserId")
    }
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "classes"]
pub struct DatabaseClass {
    /// The id of the role to use to display the class
    pub id: String,
    /// The class name
    pub name: String,
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
