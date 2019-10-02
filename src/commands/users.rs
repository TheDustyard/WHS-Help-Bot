use crate::bot_data::SqliteDatabaseConnection;
use serenity::framework::standard::{macros::command, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "List the users."]
#[sub_commands(add)]
pub fn users(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    msg.channel_id.say(
        &ctx,
        format!(
            "```\n{}```",
            crate::sample_users(
                &data
                    .get::<SqliteDatabaseConnection>()
                    .unwrap()
                    .lock()
                    .unwrap(),
            )
        ),
    )?;

    Ok(())
}

#[command]
#[description = "Register yourself"]
pub fn register() -> CommandResult {
    Err(CommandError("Unimplemented".to_owned()))
}

#[command]
#[description = "Manually add a user."]
fn add() -> CommandResult {
    Err(CommandError("Unimplemented".to_owned()))
}
