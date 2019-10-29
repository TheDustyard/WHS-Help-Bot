use lib::{
    bot_data::{BotConfig, BotLogger, DatabaseConnection},
    commands, connect_discord,
    db::Database,
    discord::framework::StandardFrameworkWrapper,
    load_config, load_environment,
    status_logger::StatusLogger,
};
use log::{debug, error, info};
use serenity::framework::standard::{DispatchError, StandardFramework};
use std::{
    env,
    sync::{Arc, Mutex, RwLock},
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
                            "The command requires at least {} argument(s), but was only given {} arguments. Make sure you provided all of the arguments.",
                            min, given
                        ),
                    );
                }
                DispatchError::TooManyArguments { max, given } => {
                    let _ = msg.channel_id.say(
                        &context,
                        &format!(
                            "The command can only take up to {} argument(s), but was given {} argument(s). Try adding qoutation marks around arguments with a space in them.",
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
        use serenity::model::id::RoleId;

        debug!("{:#?}", database.get_all_classes());
        debug!("{:#?}", database.search_classes("jeff"));
        debug!("{:#?}", database.filter_classes_by_roles(&[RoleId(10), RoleId(20), RoleId(30)]));
        debug!("{:#?}", database.get_all_groups());
        debug!("{:#?}", database.search_groups("george"));
    }

    // Persist database connection and config
    {
        let mut data = client.data.write();
        data.insert::<DatabaseConnection>(Arc::new(Mutex::new(database)));
        data.insert::<BotLogger>(StatusLogger::new(config.server.status_log));
        data.insert::<BotConfig>(config);
    }

    // Smooth Shutdown
    {
        let data = Arc::clone(&client.data);

        let shard_manager = Arc::clone(&client.shard_manager);
        let ctx = Arc::clone(&client.cache_and_http.http);

        ctrlc::set_handler(move || {
            let data = data.read();
            let status_logger = data.get::<BotLogger>().unwrap();

            if cfg!(debug_assertions) {
                let _ = status_logger.warn(
                    &ctx,
                    "Bot shutting down",
                    format!("The bot has been stopped manually and is shutting down.\n\nThis is totally normal since the bot is in debug mode and is activly under maintanance. Please do not report this."),
                );
            } else {
                let _ = status_logger.error(
                    &ctx,
                    "Bot shutting down",
                    format!("The bot has been stopped manually and is shutting down.\n\nThis is abnormal since the bot is in release mode, if the bot does not restart in the next few minutes, please report this to the bot owner `DusterTheFirst`"),
                );
            }
            
            shard_manager.lock().shutdown_all();
        })
        .expect("Error setting Ctrl-C handler");
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
