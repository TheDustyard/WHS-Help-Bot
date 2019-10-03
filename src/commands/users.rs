use crate::bot_data::SqliteDatabaseConnection;
use serenity::framework::standard::{macros::{command, group}, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

group!({
    name: "User",
    options: {
        description: "User management commands",
        prefixes: ["users", "u"],
        default_command: list
    },
    commands: [list, add],
});

#[command]
#[description = "List the users."]
#[usage = "list"]
fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
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
#[description = "Manually add a user."]
fn add() -> CommandResult {
    Err(CommandError("Unimplemented".to_owned()))
}
