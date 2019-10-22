use lib::{
    bot_data::{BotConfig, DatabaseConnection},
    commands, connect_discord,
    db::Database,
    discord::framework::StandardFrameworkWrapper,
    load_config, load_environment,
};
use log::{debug, error, info};
use serenity::framework::standard::{DispatchError, StandardFramework};
use std::{
    env,
    sync::{Arc, Mutex},
};

fn main() {
    load_environment();

    let database = match env::var("DATABASE_URL") {
        Ok(database_url) => Database::open(database_url),
        Err(e) => {
            let message = format!("Failed to load DATABASE_URL environment variable: {:?}", e);
            error!("{}", message);
            panic!("{}", message);
        }
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
        println!("{}", database);
    }

    // Persist database connection and config
    {
        let mut data = client.data.write();
        data.insert::<DatabaseConnection>(Arc::new(Mutex::new(database)));
        data.insert::<BotConfig>(config);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
