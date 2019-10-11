use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::commands::ADMIN_CHECK;
use crate::config::StaticConfiguration;
use crate::db::models::DatabaseClass;
use crate::db::schema::classes as database_classes;
use diesel::prelude::*;
use serenity::framework::standard::{
    macros::{command, group},
    ArgError, Args, CommandResult,
};
use serenity::model::{channel::Message, id::RoleId};
use serenity::prelude::*;
use serenity::utils::Colour;
use std::collections::BTreeMap;

group!({
    name: "Class",
    options: {
        description: "Class management commands",
        prefixes: ["classes", "c", "cl"],
        default_command: list
    },
    commands: [list, add, remove, edit, mine],
});

#[command]
#[description = "List the classes."]
#[usage = "[filter]"]
pub fn list(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();

    let db: &SqliteConnection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();

    let filter = args.remains();

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            match database_classes::table.load::<DatabaseClass>(db) {
                Err(err) => {
                    e.title("Error Loading Classes");
                    e.color(Colour::DARK_RED);
                    e.description(format!("```{:#?}```", err));
                }
                Ok(classes) => {
                    let classes = match filter {
                        Some(f) => {
                            let classes_len = classes.len();
                            let newclasses = classes
                                .into_iter()
                                .filter(|x| {
                                    x.tag.to_lowercase().contains(&f.to_lowercase())
                                        || x.parse_role_id()
                                            .to_role_cached(&ctx)
                                            .map(|r| {
                                                r.name.to_lowercase().contains(&f.to_lowercase())
                                            })
                                            .unwrap_or(false)
                                })
                                .collect::<Vec<_>>();

                            e.title(format!(
                                "Displaying {} of {} classes",
                                newclasses.len(),
                                classes_len
                            ));
                            e.footer(|footer| footer.text(format!("Filter: `{}`", f)));

                            newclasses
                        }
                        None => {
                            e.title(format!("Displaying {} classes", classes.len()));
                            classes
                        }
                    };

                    e.color(Colour::DARK_GREEN);

                    let mut tags = BTreeMap::<String, Vec<DatabaseClass>>::new();

                    for class in classes {
                        tags.entry(class.tag.clone())
                            .or_insert_with(|| Vec::new())
                            .push(class);
                    }

                    for (tag_name, tag_classes) in tags.iter() {
                        e.field(
                            match tag_name.len() {
                                0 => "None",
                                _ => &tag_name,
                            },
                            tag_classes
                                .iter()
                                .map(|class| {
                                    format!(
                                        "**{}**\nRole: {}\nId: `{}`",
                                        class
                                            .parse_role_id()
                                            .to_role_cached(&ctx)
                                            .map(|x| x.name)
                                            .unwrap_or_else(|| "NONEXISTANT".to_owned()),
                                        class.parse_role_id().mention(),
                                        class.id
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n"),
                            true,
                        );
                    }
                }
            };

            e
        });

        m
    })?;

    Ok(())
}

#[command]
#[description = "Add a class."]
#[usage = "<name> <tag> [role id]"]
#[example = "\"Honors History\" \"History\" 609773945796821022"]
#[checks(Admin)]
#[min_args(2)]
#[max_args(3)]
fn add(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &SqliteConnection = &data
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
        match e {
            ArgError::Parse(e) => {
                msg.channel_id
                    .say(&ctx, format!("Malformed role id: `{:?}`\n{}", e, usage))?;

                return Ok(());
            }
            _ => {}
        }
    }

    if id.is_err() {
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
    }

    Ok(())
}

#[command]
#[description = "Remove a class."]
#[usage = "<role id>"]
#[example = "609773945796821022"]
#[checks(Admin)]
#[num_args(1)]
fn remove(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &SqliteConnection = &data
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

    match id {
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
    }

    Ok(())
}

#[command]
#[description = "Edit a class."]
#[usage = "<role id> name|role|tag <new value>"]
#[example = "609773945796821022 name \"Honors History\""]
#[checks(Admin)]
#[num_args(3)]
fn edit(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &SqliteConnection = &data
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

    match id.to_role_cached(&ctx) {
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

                msg.channel_id.say(
                    &ctx,
                    format!("{:?} == {:?} = {}", role_id, role.id, role_id == role.id),
                )?;
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
    }

    Ok(())
}

#[command]
#[description = "List the classes that you have."]
#[usage = "[filter]"]
pub fn mine(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();

    let db: &SqliteConnection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();
    let config: &StaticConfiguration = data.get::<BotConfig>().unwrap();

    let filter = args.remains();

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            match database_classes::table.load::<DatabaseClass>(db) {
                Err(err) => {
                    e.title("Error Loading Classes");
                    e.color(Colour::DARK_RED);
                    e.description(format!("```{:#?}```", err));
                }
                Ok(classes) => {
                    let classes = classes
                        .into_iter()
                        .filter(|x| {
                            msg.author
                                .has_role(&ctx, config.server.id, x.parse_role_id())
                                .unwrap_or(false)
                        })
                        .collect::<Vec<_>>();
                    let classes = match filter {
                        Some(f) => {
                            let classes_len = classes.len();
                            let newclasses = classes
                                .into_iter()
                                .filter(|x| {
                                    (x.tag.to_lowercase().contains(&f.to_lowercase())
                                        || x.parse_role_id()
                                            .to_role_cached(&ctx)
                                            .map(|r| {
                                                r.name.to_lowercase().contains(&f.to_lowercase())
                                            })
                                            .unwrap_or(false))
                                })
                                .collect::<Vec<_>>();

                            e.title(format!(
                                "Displaying {} of {}",
                                newclasses.len(),
                                classes_len,
                            ));
                            e.footer(|footer| {
                                footer.text(format!(
                                    "Filter: `{}`, User: `{}`",
                                    f,
                                    msg.author.tag()
                                ))
                            });

                            newclasses
                        }
                        None => {
                            e.title(format!(
                                "Displaying {} classes",
                                classes.len()
                            ));
                            e.footer(|footer| footer.text(format!("User: `{}`", msg.author.tag())));

                            classes
                        }
                    };

                    if classes.len() == 0 {
                        e.color(Colour::DARK_GOLD);
                    } else {
                        e.color(Colour::DARK_GREEN);
                    }

                    let mut tags = BTreeMap::<String, Vec<DatabaseClass>>::new();

                    for class in classes {
                        tags.entry(class.tag.clone())
                            .or_insert_with(|| Vec::new())
                            .push(class);
                    }

                    for (tag_name, tag_classes) in tags.iter() {
                        e.field(
                            match tag_name.len() {
                                0 => "None",
                                _ => &tag_name,
                            },
                            tag_classes
                                .iter()
                                .map(|class| {
                                    format!(
                                        "**{}**\nRole: {}\nId: `{}`",
                                        class
                                            .parse_role_id()
                                            .to_role_cached(&ctx)
                                            .map(|x| x.name)
                                            .unwrap_or_else(|| "NONEXISTANT".to_owned()),
                                        class.parse_role_id().mention(),
                                        class.id
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n"),
                            true,
                        );
                    }
                }
            };

            e
        });

        m
    })?;

    Ok(())
}
