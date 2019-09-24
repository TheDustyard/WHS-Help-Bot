use crate::SqliteDatabaseConnection;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{channel::Message, id::UserId};
use serenity::prelude::*;
use std::io::Write;

#[command]
#[description = "List the classes."]
pub fn classes(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();

    // TODO: work for single class
    if let Ok(user) = args.single::<UserId>() {
        msg.channel_id
            .say(&ctx.http, format!("{:?}", user))
            .unwrap();
    }

    let mut temp = Vec::from("```rs\n".as_bytes());

    (&mut temp).write("```".as_bytes()).unwrap();

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "```\n{}```",
                crate::sample_classes(
                    &data
                        .get::<SqliteDatabaseConnection>()
                        .unwrap()
                        .lock()
                        .unwrap()
                )
            ),
        )
        .unwrap();

    Ok(())
}
