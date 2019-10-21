use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::commands::checks::ADMIN_CHECK;
use crate::config::StaticConfiguration;
use rusqlite::Connection;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::*,
};

#[command]
#[description = "Edit a class."]
#[usage = "<role id> name|role|tag <new value>"]
#[example = "609773945796821022 name \"Honors History\""]
#[checks(Admin)]
#[num_args(3)]
fn edit(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &Connection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();
    let config: &StaticConfiguration = &data.get::<BotConfig>().unwrap();

    let usage = format!(
        "Usage: `{}classes edit {}`",
        config.bot.prefix,
        EDIT_COMMAND_OPTIONS.usage.unwrap()
    );

    let id = args.single::<RoleId>()?;
    let field = args.single::<String>()?;

    /*  match id.to_role_cached(&ctx) {
        Some(role) => match &field[..] {
            "name" => {
                let name = args.single_quoted::<String>()?;
                match role.edit(&ctx, |e| e.name(&name)) {
                    Err(e) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Failed to rename role `{}`, aborting.\nError: `{:?}`",
                                role.id, e
                            ),
                        )?;
                    }
                    Ok(new_role) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Successfully renamed `{}` to `{}`.",
                                role.name, new_role.name
                            ),
                        )?;
                    }
                }
            }
            "role" => {
                let role_id = args.single::<RoleId>()?;

                match diesel::update(database_classes::table.find(role.id.to_string()))
                    .set(database_classes::id.eq(role_id.to_string()))
                    .execute(db)
                {
                    Err(e) => {
                        msg.channel_id
                            .say(&ctx, format!("Error updating role:\n`{}`", e))?;
                    }
                    Ok(_) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Successfully updated role for the class `{}` to `{}`",
                                role.name, role_id
                            ),
                        )?;
                    }
                }
            }
            "tag" => {
                let tag = args.single_quoted::<String>()?;

                match diesel::update(database_classes::table.find(role.id.to_string()))
                    .set(database_classes::tag.eq(&tag))
                    .execute(db)
                {
                    Err(e) => {
                        msg.channel_id
                            .say(&ctx, format!("Error updating tag:\n`{}`", e))?;
                    }
                    Ok(_) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Successfully updated tag for the class `{}` to `{}`",
                                role.name, tag
                            ),
                        )?;
                    }
                };
            }
            x => {
                msg.channel_id.say(
                    &ctx,
                    format!("{} is an invalid property to modify\n{}", x, usage),
                )?;
            }
        },
        None => {
            msg.channel_id
                .say(&ctx, format!("Role `{}` does not exist.", id))?;
        }
    } */

    Ok(())
}
