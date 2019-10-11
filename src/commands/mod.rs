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

pub use classes::CLASS_GROUP;
use crate::config::StaticConfiguration;
use crate::bot_data::BotConfig;

#[help]
#[max_levenshtein_distance(2)]
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
    name: "Admin",
    options: {
        description: "Administrative commands",
        checks: [Admin],
        only_in: "guilds",
    },
    commands: [say],
});

#[command]
#[owners_only]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest())?;
    msg.delete(&ctx)?;

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