use super::super::checks::ADMIN_CHECK;
use super::super::embeds::create_embed_for_groups;
use crate::bot_data::{BotConfig, BotLogger, DatabaseConnection};
use crate::db::Database;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::{ChannelType, Message},
    prelude::*,
    utils::Colour,
};
use std::convert::TryInto;

group!({
    name: "Groups",
    options: {
        description: "Group management commands",
        prefixes: ["groups", "g", "gr"],
        // default_command: list
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
#[usage = "{detailed} [filter..]"]
#[example = "History"]
#[sub_commands(detailed)]
fn list(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    list_command_internal(ctx, msg, args, false)
}

#[command]
#[description = "List the groups that are avaliable with extra detailed information"]
#[usage = "[filter..]"]
#[example = "History"]
fn detailed(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    list_command_internal(ctx, msg, args, true)
}

#[command]
#[description = "Create a group"]
#[usage = "<name>"]
#[example = "History"]
#[checks(Admin)]
#[num_args(1)]
fn create(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &Database = &data.get::<DatabaseConnection>().unwrap().lock().unwrap();
    let config = data.get::<BotConfig>().unwrap();
    let logger = data.get::<BotLogger>().unwrap();

    let name = args.single_quoted::<String>()?.replace(" ", "-");
    let channel_group = match config.server.id.create_channel(&ctx, |channel| {
        channel.name(&name).kind(ChannelType::Category)
    }) {
        Ok(cg) => cg,
        Err(err) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Error");
                    e.description(format!("Encountered an error when creating the channel group `{}`, aborting.\n```rs\n{:?}```", name, err));
                    e.color(Colour::DARK_RED);

                    e
                });

                m
            })?;
            return Ok(());
        }
    };

    let vc = match config.server.id.create_channel(&ctx, |channel| {
        channel
            .name(format!("{}-help", &name))
            .category(channel_group.id)
            .kind(ChannelType::Voice)
    }) {
        Ok(vc) => vc,
        Err(err) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Error");
                    e.description(format!("Encountered an error when creating the voice chat for the new group, aborting.\n```rs\n{:?}```", err));
                    e.color(Colour::DARK_RED);

                    e
                });

                m
            })?;
            return Ok(());
        }
    };

    match db.insert_group(&name, channel_group.id, vc.id) {
        Ok(group) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title(format!("Successfully created new group {}", group.name));
                    e.description(format!(
                        "
                        Name: {}
                        Channel Group: {}
                        Voice Chat: {}
                        Id: `{}`",
                        group.name,
                        group.channel_group.mention(),
                        group.vc.mention(),
                        group.id
                    ));
                    e.color(Colour::DARK_GREEN);

                    e
                });

                m
            })?;

            //TODO: LOG
            logger.success(
                &ctx,
                format!("{} created group `{}`", msg.author.tag(), group.name),
                format!(
                    "
                        Name: {}
                        Channel Group: {}
                        Voice Chat: {}
                        Id: `{}`",
                    group.name,
                    group.channel_group.mention(),
                    group.vc.mention(),
                    group.id
                ),
            )?;
        }
        Err(err) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Error");
                    e.description(format!(
                        "Encountered an error when saving the new group, aborting.\n```rs\n{:?}```",
                        err
                    ));
                    e.color(Colour::DARK_RED);

                    e
                });

                m
            })?;

            match channel_group.delete(&ctx) {
                Ok(_) => {}
                Err(err) => {
                    msg.channel_id.send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Error");
                            e.description(format!(
                                "Encountered an error when deleting the created channel group, `{}`, you will have to do so manually.\n```rs\n{:?}```",
                                channel_group, err
                            ));
                            e.color(Colour::DARK_RED);

                            e
                        });

                        m
                    })?;
                }
            };
            match vc.delete(&ctx) {
                Ok(_) => {}
                Err(err) => {
                    msg.channel_id.send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Error");
                            e.description(format!(
                                "Encountered an error when deleting the created voice channel, `{}`, you will have to do so manually.\n```rs\n{:?}```",
                                vc, err
                            ));
                            e.color(Colour::DARK_RED);

                            e
                        });

                        m
                    })?;
                }
            };
        }
    }

    Ok(())
}

#[command]
#[description = "Import a group"]
#[usage = "TODO:"]
#[example = "TODO:"]
#[checks(Admin)]
// #[min_args(0)]
// #[max_args(1)]
fn import(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

#[command]
#[description = "Delete a group"]
#[usage = "TODO:"]
#[example = "TODO:"]
#[checks(Admin)]
// #[min_args(0)]
// #[max_args(1)]
fn delete(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}
