#[macro_use]
extern crate diesel_migrations;

use serenity::framework::standard::StandardFramework;
use std::sync::{Arc, Mutex};

use bot_framework::*;

embed_migrations!("./migrations");

fn main() {
    load_environment();

    let connection = establish_connection();

    // Setup database
    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    sample_users(&connection, &mut std::io::stdout());
    sample_classes(&connection, &mut std::io::stdout());

    let config = load_config().unwrap();

    println!("{:#?}", config);

    let mut client = connect_discord();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .help(&commands::HELP_COMMAND) // Help
            .group(&commands::GENERAL_GROUP),
    );

    // Persist database connection
    {
        let mut data = client.data.write();
        data.insert::<SqliteDatabaseConnection>(Arc::new(Mutex::new(connection)));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
