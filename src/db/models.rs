use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serenity::model::id::UserId;
use uuid::Uuid;

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
    pub fn get_classes_ids(&self) -> Vec<Uuid> {
        self.classes
            .split(",")
            .map(|x| Uuid::parse_str(x).expect("Malformed uuid in user classes"))
            .collect()
    }

    pub fn get_classes(&self, connection: &SqliteConnection) -> Vec<DatabaseClass> {
        use crate::db::schema::classes::dsl::*;

        self.classes
            .split(",")
            .map(|x| classes.find(x).first(connection).unwrap())
            .collect()
    }

    pub fn get_id(&self) -> UserId {
        UserId(
            self.id
                .parse::<u64>()
                .expect("Unparsable u64 UserId in database"),
        )
    }
}

#[derive(Queryable, Debug)]
pub struct DatabaseClass {
    /// The uuid of the class
    id: String,
    /// The class name
    pub name: String,
}

impl DatabaseClass {
    pub fn get_id(&self) -> Uuid {
        Uuid::parse_str(&self.id).expect("Malformed class id")
    }
}
