use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::commands::ADMIN_CHECK;
use crate::config::StaticConfiguration;
use crate::db::models::DatabaseClass;
use crate::db::schema::classes as database_classes;
use diesel::prelude::*;
use prettytable::{cell, row, Table};
use serenity::framework::standard::{
    macros::{command, group},
    ArgError, Args, CommandResult,
};
use serenity::model::{channel::Message, id::RoleId};
use serenity::prelude::*;
use std::fmt::Write;

group!({
    name: "Class",
    options: {
        description: "Class management commands",
        prefixes: ["classes", "c"],
        default_command: list,
        checks: [Admin]
    },
    commands: [list, add],
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

    let mut output = String::new();
    writeln!(output, "```md").unwrap();

    {
        let classes = database_classes::table.load::<DatabaseClass>(db)?;

        let classes = match filter {
            Some(f) => classes
                .into_iter()
                .filter(|x| x.name.to_lowercase().contains(&f.to_lowercase()))
                .collect::<Vec<_>>(),
            None => classes,
        };

        match filter {
            Some(f) => writeln!(
                output,
                "# Displaying {} classes with filter `{}`",
                classes.len(),
                f
            )
            .unwrap(),
            None => writeln!(output, "# Displaying {} classes", classes.len()).unwrap(),
        };

        let mut table = Table::new();
        table.add_row(row!["NAME", "CLASS ROLE"]);

        for class in classes {
            table.add_row(row![
                class.name,
                class
                    .parse_role_id()
                    .to_role_cached(&ctx)
                    .map(|r| if class.name == r.name {
                        format!("{}", class.id)
                    } else {
                        format!("{} ({})", class.id, r.name)
                    })
                    .unwrap_or_else(|| format!("(NONEXISTANT) {}", class.id)),
            ]);
        }

        writeln!(output, "{}", table).unwrap();
    }

    writeln!(output, "```").unwrap();

    msg.channel_id.say(&ctx, output).unwrap();

    Ok(())
}

#[command]
#[description = "Manually add a class."]
#[usage = "<name> [role id]"]
#[example = "\"Honors History\" 609773945796821022 "]
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

    if args.remaining() == 0 {
        msg.channel_id
            .say(&ctx, format!("Too few arguments\n{}", usage))?;

        return Ok(());
    }

    let name = args.single_quoted::<String>();
    let id = args.single::<RoleId>();

    if name.is_err() {
        msg.channel_id
            .say(&ctx, format!("Malformed name argument\n{}", usage))?;
        return Ok(());
    }
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

    let name = name.unwrap();

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
                        name: name.clone(),
                    })
                    .execute(db);

                match classes_result {
                    Ok(_) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Created a new Discord role {mention} for the class {classname}\nTo edit this class use `!classes edit {id}`\nTo delete this class use `!classes delete {id}`\nOr, you can just mention the role if that is easier",
                                mention=r.mention(), id=r.id, classname=name
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
                        name: name.clone(),
                    })
                    .execute(db);

                match classes_result {
                    Ok(_) => {
                        msg.channel_id.say(
                            &ctx,
                            format!(
                                "Added existing Discord role {mention} to the class {classname}\nTo edit this class use `!classes edit {id}`\nTo delete this class use `!classes delete {id}`\nOr, you can just mention the role if that is easier",
                                mention=role.mention(), id=role.id, classname=name
                            ),
                        )?;
                    }
                    Err(e) => {
                        use diesel::result::{Error, DatabaseErrorKind};
                        match e {
                            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                                msg.channel_id.say(&ctx, "A class already exists with that ID")?;
                            },
                            _ => {
                                msg.channel_id.say(&ctx, format!("Error saving class: {}", e))?;
                            }
                        };
                    }
                }
            }
        }
    }

    Ok(())
}
