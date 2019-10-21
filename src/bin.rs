use lib::{
    bot_data::{BotConfig, SqliteDatabaseConnection},
    commands, connect_discord,
    db::{
        self,
        model::{Category, Class},
        Migrateable
    },
    discord::framework::StandardFrameworkWrapper,
    load_config, load_environment,
};
use log::{debug, error, info};
use rusqlite::NO_PARAMS;
use serenity::framework::standard::{DispatchError, StandardFramework};
use std::sync::{Arc, Mutex};

fn main() {
    load_environment();
    let connection = db::establish_connection();
    // TODO: determine when to do so
    match db::AllTables::migrate_up(&connection) {
        Ok(()) => debug!("Successfully ran up migrations"),
        Err(e) => error!("Failed to run up migration: {:?}", e),
    };

    let config = load_config();

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

    info!(
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

    {
        let mut statement = connection.prepare("SELECT * FROM category").unwrap();
        let category_iter = statement.query_map(NO_PARAMS, Category::from_row).unwrap();
        for category in category_iter {
            println!("Found cat {:?}", category.unwrap());
        }

        let mut statement = connection.prepare("SELECT * FROm class").unwrap();
        let class_iter = statement.query_map(NO_PARAMS, Class::from_row).unwrap();
        for class in class_iter {
            println!("Found class {:?}", class.unwrap());
        }
    }

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
