use super::super::checks::ADMIN_CHECK;
use super::super::embeds::create_embed_for_classes;
use crate::bot_data::{BotConfig, BotLogger, DatabaseConnection};
use crate::db::Database;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
    utils::Colour,
};
use std::convert::TryInto;

group!({
    name: "Classes",
    options: {
        description: "Class management commands",
        prefixes: ["classes", "c", "cl"],
        // default_command: list
    },
    commands: [list/*, add, remove, edit, mine, join, leave */],
});

/// The internal structure of the list command, shared by !list and !list detailed
fn list_command_internal(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    detailed: bool,
) -> CommandResult {
    let data = ctx.data.read();
    let db: &Database = &data.get::<DatabaseConnection>().unwrap().lock().unwrap();

    let filter = args.remains();

    let all_count = db.classes_count()?;

    let classes = match &filter {
        Some(filter) => db.search_classes(filter),
        None => db.get_all_classes(),
    };

    match classes {
        Ok(classes) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    create_embed_for_classes(
                        e,
                        all_count.try_into().unwrap(),
                        &classes,
                        filter,
                        detailed,
                    )
                });
                m
            })?;
        }
        Err(err) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Error Loading Classes");
                    e.color(Colour::DARK_RED);
                    e.description(format!("```{:#?}```", err));

                    e
                });
                m
            })?;
        }
    };

    Ok(())
}

#[command]
#[description = "List the classes that are avaliable"]
#[usage = "{detailed} <filter>"]
#[example = "History"]
#[min_args(0)]
#[max_args(1)]
#[sub_commands(detailed)]
fn list(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    list_command_internal(ctx, msg, args, false)
}

#[command]
#[description = "List the classes that are avaliable with extra detailed information"]
#[usage = "<filter>"]
#[example = "History"]
#[min_args(0)]
#[max_args(1)]
fn detailed(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    list_command_internal(ctx, msg, args, true)
}
