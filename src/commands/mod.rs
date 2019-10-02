use serenity::prelude::*;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{check, command, group, help},
        Args, CheckResult, CommandGroup, CommandOptions, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

pub mod classes;
pub mod users;
pub mod errors;

use classes::CLASSES_COMMAND;
use errors::ERRORS_COMMAND;
use users::{REGISTER_COMMAND, USERS_COMMAND};
use crate::config::StaticConfiguration;
use crate::bot_data::BotConfig;

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

group!({
    name: "Admin",
    options: {
        description: "Administrative commands",
        checks: [Admin],
        only_in: "guilds",
    },
    commands: [errors],
});

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[check]
#[name = "Admin"]
#[check_in_help(true)]
#[display_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    let data = ctx.data.read();
    let config: &StaticConfiguration = data.get::<BotConfig>().unwrap();

    if let Some(member) = msg.member(&ctx.cache) {
        if let Some(roles) = member.roles(&ctx.cache) {
            // Check if user has any admin role
            return config.server.admin_roles.iter().any(|x| roles.iter().any(|y| x == &y.id)).into();
        }
    }

    false.into()
}
