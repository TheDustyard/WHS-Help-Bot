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

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

// #[command]
// fn users(ctx: &mut Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "h!")?;

//     Ok(())
//     // help_commands::with_embeds(context, msg, args, help_options, groups, owners)
// }
