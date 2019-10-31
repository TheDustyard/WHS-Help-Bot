use super::super::checks::ADMIN_CHECK;
use super::super::embeds::create_embed_for_groups;
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
    name: "Groups",
    options: {
        description: "Group management commands",
        prefixes: ["groups", "g", "gr"],
        default_command: list
    },
    commands: [list, create, import/*, remove, edit, mine, join, leave */],
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

    let all_count = db.groups_count()?;

    let groups = match &filter {
        Some(filter) => db.search_groups(filter),
        None => db.get_all_groups(),
    };

    match groups {
        Ok(groups) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    create_embed_for_groups(
                        e,
                        all_count.try_into().unwrap(),
                        &groups,
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
                    e.title("Error Loading Groups");
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
#[description = "List the groups that are avaliable"]
#[usage = "{detailed} <filter>"]
#[example = "History"]
#[min_args(0)]
#[max_args(1)]
#[sub_commands(detailed)]
fn list(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    list_command_internal(ctx, msg, args, false)
}

#[command]
#[description = "List the groups that are avaliable with extra detailed information"]
#[usage = "<filter>"]
#[example = "History"]
#[min_args(0)]
#[max_args(1)]
fn detailed(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    list_command_internal(ctx, msg, args, true)
}

#[command]
#[description = "Create a group"]
#[usage = "TODO:"]
#[example = "TODO:"]
// #[min_args(0)]
// #[max_args(1)]
fn create(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

#[command]
#[description = "Import a group"]
#[usage = "TODO:"]
#[example = "TODO:"]
// #[min_args(0)]
// #[max_args(1)]
fn import(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}