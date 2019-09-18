use uuid::Uuid;
use serenity::model::id::UserId;

#[derive(Queryable, Debug)]
pub struct DatabaseUser {
    /// The discord ID of the user
    id: String,
    /// The users display name
    pub name: String,
    /// The ids of the classes that the user is in seperated by commas
    classes: String
}

impl DatabaseUser {
    pub fn get_classes(&self) -> Vec<Uuid> {
        self.classes.split(",").map(|x| Uuid::parse_str(x).expect("Malformed uuid in user classes")).collect()
    }

    pub fn get_id(&self) -> UserId {
        UserId(self.id.parse::<u64>().expect("Unparsable u64 UserId in database"))
    }
}

#[derive(Queryable, Debug)]
pub struct DatabaseClass {
    /// The uuid of the class
    id: String,
    /// The class name
    pub name: String
}

impl DatabaseClass {
    pub fn get_id(&self) -> Uuid {
        Uuid::parse_str(&self.id).expect("Malformed class id")
    }
}