use crate::SqliteDatabaseConnection;
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::{channel::Message, id::UserId};
use serenity::prelude::*;
use std::collections::HashSet;
use std::io::Write;

#[help]
#[command_not_found_text = "Could not find: `{}`."]
fn help_command(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, &help_options, groups, owners)
}

group!({
    name: "General",
    options: {
        description: "General commands"
    },
    commands: [ping, classes, users],
});

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn classes(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let mut temp = Vec::from("```rs\n".as_bytes());

    crate::sample_classes(
        &data
            .get::<SqliteDatabaseConnection>()
            .unwrap()
            .lock()
            .unwrap(),
        &mut temp,
    );

    (&mut temp).write("```".as_bytes()).unwrap();

    msg.channel_id
        .say(&ctx.http, std::str::from_utf8(&temp).unwrap())
        .unwrap();

    Ok(())
}

#[command]
fn users(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let mut temp = Vec::from("```rs\n".as_bytes());

    crate::sample_users(
        &data
            .get::<SqliteDatabaseConnection>()
            .unwrap()
            .lock()
            .unwrap(),
        &mut temp,
    );

    (&mut temp).write("```".as_bytes()).unwrap();

    msg.channel_id
        .say(&ctx.http, std::str::from_utf8(&temp).unwrap())
        .unwrap();

    Ok(())
}
