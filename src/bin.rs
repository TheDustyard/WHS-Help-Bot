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

use bot_framework::db;
use bot_framework::db::models::DatabaseUser;
use bot_framework::establish_connection;

embed_migrations!("./migrations");

struct Handler;

impl EventHandler for Handler {}

fn main() {
    use db::schema::users::dsl::*;

    let connection = establish_connection();

    // Setup database
    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let results = users
        .limit(5)
        .load::<DatabaseUser>(&connection)
        .expect("Error loading users");

    println!("Displaying {} of {} users", results.len(), users.count().get_result::<i64>(&connection).unwrap());
    for user in results {
        println!("{}: {}", user.id, user.name);
    }

    // // Open file
    // let config_file = fs::read_to_string("./config.toml").expect("Failed to open ./config.json");
    // // Load config
    // let config: config::StaticConfiguration = toml::from_str(&config_file).unwrap();

    // println!("{:#?}", config);

    // // Login with a bot token from the environment
    // let mut client = Client::new(
    //     &env::var("TOKEN").expect("Missing TOKEN environment variable"),
    //     Handler,
    // )
    // .expect("Error creating client");
    // client.with_framework(
    //     StandardFramework::new()
    //         .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
    //         .help(&commands::HELP_COMMAND) // Help
    //         .group(&commands::GENERAL_GROUP),
    // );

    // // start listening for events by starting a single shard
    // if let Err(why) = client.start() {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
}
