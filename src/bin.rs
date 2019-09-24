#[macro_use]
extern crate diesel_migrations;

use serenity::framework::standard::{DispatchError, StandardFramework};
use std::sync::{Arc, Mutex};

use lib::*;

embed_migrations!("./migrations");

fn main() {
    load_environment();

    let connection = establish_connection();

    // Setup database
    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    println!("{}", sample_users(&connection));
    println!("{}", sample_classes(&connection));

    let config = load_config().unwrap();

    println!("{:#?}", config);

    let mut client = connect_discord();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix(&config.bot.prefix)
                    .owners(vec![config.bot.owner].into_iter().collect())
            }) // set the bot's prefix to "!"
            .help(&commands::HELP_COMMAND) // Help
            .group(&commands::GENERAL_GROUP)
            .group(&commands::MANAGEMENT_GROUP)
            .on_dispatch_error(|context, msg, error| match error {
                DispatchError::NotEnoughArguments { min, given } => {
                    let s = format!("Need {} arguments, but only got {}.", min, given);

                    let _ = msg.channel_id.say(&context.http, &s);
                }
                DispatchError::TooManyArguments { max, given } => {
                    let s = format!("Max arguments allowed is {}, but got {}.", max, given);

                    let _ = msg.channel_id.say(&context.http, &s);
                }
                _ => println!("Unhandled dispatch error."),
            })
            .after(|ctx, msg, cmd_name, error| {
                //  Print out an error if it happened
                if let Err(why) = error {
                    println!("Error in {}: {:?}", cmd_name, why);
                    msg.channel_id
                        .say(&ctx.http, format!("Error in {}: {:?}", cmd_name, why))
                        .unwrap();
                }
            }),
    );

    // Persist database connection
    {
        let mut data = client.data.write();
        data.insert::<SqliteDatabaseConnection>(Arc::new(Mutex::new(connection)));
    }

    println!(
        "Starting bot {:?} with prefix {} and owner {:?}",
        // TODO: more idk
        client
            .cache_and_http
            .http
            .get_current_application_info()
            .unwrap()
            .name,
        config.bot.prefix,
        config.bot.owner
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
