use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::commands::checks::ADMIN_CHECK;
use crate::config::StaticConfiguration;
use crate::db::model::Class;
use rusqlite::Connection;
use serenity::{
    framework::standard::{macros::command, ArgError, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::*,
};

#[command]
#[description = "Add a class."]
#[usage = "<name> <tag> [role id]"]
#[example = "\"Honors History\" \"History\" 609773945796821022"]
#[checks(Admin)]
#[min_args(2)]
#[max_args(3)]
fn add(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &Connection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();
    let config: &StaticConfiguration = &data.get::<BotConfig>().unwrap();

    let usage = format!(
        "Usage: `{}classes add {}`",
        config.bot.prefix,
        ADD_COMMAND_OPTIONS.usage.unwrap()
    );

    let name = args.single_quoted::<String>()?;
    let tag = args.single_quoted::<String>()?;
    let id = args.single::<RoleId>();

    if let Err(e) = &id {
        if let ArgError::Parse(e) = e {
            msg.channel_id
                .say(&ctx, format!("Malformed role id: `{:?}`\n{}", e, usage))?;

            return Ok(());
        }
    }

    /* if id.is_err() {
        let role_result = config
            .server
            .id
            .create_role(&ctx, |r| r.hoist(true).mentionable(true).name(&name));
        match role_result {
            Ok(mut r) => {
                let classes_result = diesel::insert_or_ignore_into(database_classes::table)
                    .values(DatabaseClass {
                        id: r.id.to_string(),
                        tag: tag.clone(),
                    })
                    .execute(db);

                match classes_result {
                    Ok(_) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Created a new Discord role {mention} for the class {classname} with the tag {tag}\nTo edit this class use `!classes edit {id}`\nTo delete this class use `!classes delete {id}`\nOr, you can just mention the role if that is easier",
                                mention=r.mention(), id=r.id, classname=name, tag=tag
                            ),
                        )?;
                    }
                    // UNWIND
                    Err(e) => {
                        msg.channel_id
                            .say(&ctx, format!("Error saving class: {}", e))?;
                        match r.delete(&ctx) {
                            Ok(_) => {}
                            Err(e) => {
                                msg.channel_id
                                    .say(&ctx, format!("Error deleting role: {}", e))?;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                msg.channel_id.say(&ctx, format!("{:?}", e))?;
            }
        };
    } else {
        let id = id.unwrap();
        match id.to_role_cached(&ctx) {
            None => {
                msg.channel_id.say(
                    &ctx,
                    format!(
                        "Could not create class `{}`. No role with the ID `{}` exists",
                        name, id
                    ),
                )?;
            }
            Some(role) => {
                let classes_result = diesel::insert_into(database_classes::table)
                    .values(DatabaseClass {
                        id: role.id.to_string(),
                        tag: tag.clone(),
                    })
                    .execute(db);

                match classes_result {
                    Ok(_) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Added existing Discord role {mention} to the class {classname} with the tag {tag}\nTo edit this class use `!classes edit {id}`\nTo delete this class use `!classes delete {id}`\nOr, you can just mention the role if that is easier",
                                mention=role.mention(), id=role.id, classname=name, tag=tag
                            ),
                        )?;
                    }
                    Err(e) => {
                        use diesel::result::{DatabaseErrorKind, Error};
                        match e {
                            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                                msg.channel_id
                                    .say(&ctx, "A class already exists with that ID")?;
                            }
                            _ => {
                                msg.channel_id
                                    .say(&ctx, format!("Error saving class: {}", e))?;
                            }
                        };
                    }
                }
            }
        }
    } */

    Ok(())
}
