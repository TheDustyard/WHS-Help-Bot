use serenity::{
    framework::standard::{
        macros::{check},
        Args, CheckResult, CommandOptions
    },
    model::channel::Message,
    prelude::*
};

use crate::config::StaticConfiguration;
use crate::bot_data::BotConfig;

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