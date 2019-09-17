use serenity::model::{
    channel::Message,
    id::UserId
};
use serenity::prelude::*;
use serenity::framework::standard::{
    Args,
    HelpOptions,
    CommandGroup,
    CommandResult,
    help_commands,
    macros::{
        command,
        group,
        help
    }
};
use std::collections::HashSet;

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

group!({
    name: "General",
    options: {},
    commands: [ping],
});

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}