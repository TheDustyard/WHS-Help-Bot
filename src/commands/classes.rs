use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::commands::ADMIN_CHECK;
use crate::db::models::DatabaseClass;
use crate::db::schema::classes as database_classes;
use diesel::prelude::*;
use prettytable::{cell, row, Table};
use serenity::framework::standard::{
    macros::{command, group},
    ArgError, Args, CommandError, CommandResult,
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
                    .parse_role()
                    .to_role_cached(&ctx)
                    .map(|r| if class.name == r.name {
                        format!("{}", class.role)
                    } else {
                        format!("{} ({})", class.role, r.name)
                    })
                    .unwrap_or_else(|| format!("! {}", class.role)),
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
    let config = &data.get::<BotConfig>().unwrap();

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

    msg.channel_id.say(&ctx, format!("{:?}, {:?}", name, id))?;

    Ok(())
}
