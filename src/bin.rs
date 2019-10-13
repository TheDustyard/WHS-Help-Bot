#[macro_use]
extern crate diesel_migrations;

use serenity::framework::standard::{DispatchError, StandardFramework};
use std::sync::{Arc, Mutex};

use lib::bot_data::{BotConfig, SqliteDatabaseConnection};
use lib::{
    commands, connect_discord, establish_connection, load_config, load_environment,
    StandardFrameworkWrapper,
};

embed_migrations!("./migrations");

fn main() {
    load_environment();

    let connection = establish_connection();

    // Setup database
    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    // println!("{}", sample_users(&connection));
    // println!("{}", sample_classes(&connection));

    let config = load_config().unwrap();

    // println!("{:#?}", config);

    let mut client = connect_discord();
    client.with_framework(
        StandardFrameworkWrapper::wrap(
            StandardFramework::new()
            .configure(|c| {
                c.prefix(&config.bot.prefix)
                    .owners(vec![config.bot.owner].into_iter().collect())
            }) // set the bot's prefix to "!"
            .help(&commands::HELP_COMMAND) // Help
            .group(&commands::CLASS_GROUP)
            .group(&commands::ADMIN_GROUP)
            .on_dispatch_error(|context, msg, error| match error {
                DispatchError::NotEnoughArguments { min, given } => {
                    let _ = msg.channel_id.say(
                        &context,
                        &format!(
                            "Needed {} arguments, but only got {} arguments. Try adding qoutation marks around arguments with a space in them.",
                            min, given
                        ),
                    );
                }
                DispatchError::TooManyArguments { max, given } => {
                    let _ = msg.channel_id.say(
                        &context,
                        &format!(
                            "Max arguments allowed is {}, but got {} arguments. Try adding qoutation marks around arguments with a space in them.",
                            max, given
                        ),
                    );
                }
                DispatchError::CheckFailed(s, _) => {
                    let _ = msg.channel_id.say(
                        &context,
                        &format!("You cannot run this command, the `{}` check failed.", s),
                    );
                }
                DispatchError::LackingPermissions(p) => {
                    let _ = msg.channel_id.say(
                        &context,
                        &format!("You are lacking the `{:?}` permission(s).", p),
                    );
                }
                DispatchError::OnlyForDM => {
                    let _ = msg
                        .channel_id
                        .say(&context, "This command can only be run in DMs");
                }
                DispatchError::OnlyForGuilds => {
                    let _ = msg
                        .channel_id
                        .say(&context, "This command can only be run in guilds.");
                }
                _ => println!("Unhandled dispatch error."),
            })
            .after(|ctx, msg, cmd_name, error| {
                //  Print out an error if it happened
                if let Err(why) = error {
                    println!("Error in {}: {:?}", cmd_name, why);
                    msg.channel_id
                        .say(&ctx, format!("Error in {}: {:?}", cmd_name, why))
                        .unwrap();
                }
            })
        )
    );

    println!(
        "Starting bot {:?} with prefix {} and owner {}",
        client
            .cache_and_http
            .http
            .get_current_application_info()
            .unwrap()
            .name,
        config.bot.prefix,
        config.bot.owner.to_string()
    );

    // Persist database connection and config
    {
        let mut data = client.data.write();
        data.insert::<SqliteDatabaseConnection>(Arc::new(Mutex::new(connection)));
        data.insert::<BotConfig>(config);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
