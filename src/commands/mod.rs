use serenity::prelude::*;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

pub mod classes;
pub mod users;

use classes::CLASSES_COMMAND;
use users::{REGISTER_COMMAND, USERS_COMMAND};

#[help]
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
    commands: [ping],
});

group!({
    name: "Management",
    options: {
        description: "User management commands"
    },
    commands: [users, classes, register],
});

// TODO: error checks, shid like that
// group!({
//     name: "Admin",
//     options: {
//         description: "Administrative commands",
//         checks: // TODO:
//     },
//     commands: [],
// });

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
