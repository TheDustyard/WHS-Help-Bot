use crate::bot_data::SqliteDatabaseConnection;
use crate::commands::ADMIN_CHECK;
use crate::db::models::DatabaseClass;
use crate::db::schema::classes as database_classes;
use diesel::prelude::*;
use prettytable::{cell, row, Table};
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::fmt::Write;

#[command]
#[description = "List the classes."]
#[usage = "[filter]"]
#[checks(Admin)]
#[sub_commands(add, /* remove, edit */)]
pub fn classes(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
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
            Some(f) => writeln!(output, "# Displaying {} classes with filter `{}`", classes.len(), f).unwrap(),
            None => writeln!(output, "# Displaying {} classes", classes.len()).unwrap()
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
                    .unwrap_or(format!("! {}", class.role)),
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
fn add() -> CommandResult {
    Err(CommandError("Unimplemented".to_owned()))
}
