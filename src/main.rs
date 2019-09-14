use serenity::client::Client;
use serenity::model::{
    channel::Message,
    id::UserId
};
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    Args,
    HelpOptions,
    CommandGroup,
    help_commands,
    macros::{
        command,
        group,
        help
    }
};
use std::fs::File;
use std::collections::HashSet;

group!({
    name: "General",
    options: {},
    commands: [ping],
});

mod config;

struct Handler;

impl EventHandler for Handler {}

#[help]
#[command_not_found_text = "Could not find: `{}`."]
fn help_command(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, &help_options, groups, owners)
}

fn main() {
    // Open file
    let config_reader = File::open("./config.json").expect("Failed to open ./config.json");
    // Load config
    let config: config::StaticConfiguration = serde_json::from_reader(config_reader).unwrap();

    println!("{:#?}", config);

    // Login with a bot token from the environment
    let mut client = Client::new(&config.token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .help(&HELP_COMMAND)// Help
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}