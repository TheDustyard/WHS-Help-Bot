#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::user::User;
use serenity::prelude::*;
use std::env;
use std::fs;

use crate::db::models::{DatabaseClass, DatabaseUser};

pub mod commands;
pub mod config;
pub mod db;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn sample_users(connection: &SqliteConnection) {
    use db::schema::users::dsl::*;

    let results = users
        .limit(5)
        .load::<DatabaseUser>(connection)
        .expect("Error loading users");

    println!(
        "Displaying {} of {} users",
        results.len(),
        users.count().get_result::<i64>(connection).unwrap()
    );
    for user in results {
        println!("{:#?}", user);
    }
}

pub fn sample_classes(connection: &SqliteConnection) {
    use db::schema::classes::dsl::*;

    let results = classes
        .limit(5)
        .load::<DatabaseClass>(connection)
        .expect("Error loading users");

    println!(
        "Displaying {} of {} classes",
        results.len(),
        classes.count().get_result::<i64>(connection).unwrap()
    );
    for class in results {
        println!("{:?} | {}", class, class.get_id());
    }
}
