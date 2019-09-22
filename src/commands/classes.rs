use crate::SqliteDatabaseConnection;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::{channel::Message};
use serenity::prelude::*;
use std::io::Write;

group!({
    name: "Classes",
    options: {
        description: "Class management commands",
        prefixes: ["classes", "c"],
        // default_command: list,
    },
    commands: [list],
});

#[command]
#[description = "List the classes."]
fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
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