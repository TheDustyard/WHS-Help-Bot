use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::commands::checks::ADMIN_CHECK;
use crate::config::StaticConfiguration;
use rusqlite::Connection;
use serenity::{
    framework::standard::{macros::command, ArgError, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::*,
};

#[command]
#[description = "Remove a class."]
#[usage = "<role id>"]
#[example = "609773945796821022"]
#[checks(Admin)]
#[num_args(1)]
fn remove(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &Connection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();

    let config: &StaticConfiguration = &data.get::<BotConfig>().unwrap();

    let usage = format!(
        "Usage: `{}classes remove {}`",
        config.bot.prefix,
        REMOVE_COMMAND_OPTIONS.usage.unwrap()
    );

    let id = args.single::<RoleId>();

    /* match id {
        Err(e) => match e {
            ArgError::Parse(e) => {
                msg.channel_id
                    .say(&ctx, format!("Malformed role id: `{:?}`\n{}", e, usage))?;

                return Ok(());
            }
            _ => {
                msg.channel_id.say(
                    &ctx,
                    format!("Unknown error parsing RoleID: `{:?}`\n{}", e, usage),
                )?;
            }
        },
        Ok(id) => {
            let class_result =
                diesel::delete(database_classes::table.find(id.to_string())).execute(db);
            match class_result {
                Err(e) => {
                    msg.channel_id.say(
                        &ctx,
                        format!(
                            "Could not delete class `{id}`. No class with the ID `{id}` exists\n{err}",
                            id=id, err=e
                        ),
                    )?;
                }
                Ok(_) => match id.to_role_cached(&ctx) {
                    Some(mut role) => match role.delete(&ctx) {
                        Ok(_) => {
                            msg.channel_id.say(
                                &ctx,
                                format!("Deleted class `{}` and its ascociated role", id),
                            )?;
                        }
                        Err(e) => {
                            msg.channel_id.say(
                                    &ctx,
                                    format!(
                                        "Deleted class `{}` but could not delete its ascociated role: ```rs\n{:?}```",
                                        id, e
                                    ),
                                )?;
                        }
                    },
                    None => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Deleted class `{}` but could not find its ascociated role",
                                id
                            ),
                        )?;
                    }
                },
            }
        }
    } */

    Ok(())
}
