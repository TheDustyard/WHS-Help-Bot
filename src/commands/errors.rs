use crate::bot_data::{BotConfig, SqliteDatabaseConnection};
use crate::db::models::{DatabaseClass, DatabaseUser};
use diesel::prelude::*;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::error::Error;
use std::fmt::Write;

#[command]
#[description = "Check for errors and problems with the user listing."]
// #[usage = "[filter]"]
#[only_in(guilds)]
fn errors(ctx: &mut Context, msg: &Message) -> CommandResult {
    use crate::db::schema::{classes, users};

    let data = ctx.data.read();

    // let config = data.get::<BotConfig>().unwrap();

    let db: &SqliteConnection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();

    let usersresults = users::table
        .load::<DatabaseUser>(db)
        .expect("Error loading users");

    let classesresults = classes::table
        .load::<DatabaseClass>(db)
        .expect("Error loading classes");

    let mut output = String::new();

    writeln!(output, "> **User Database Errors**").unwrap();
    writeln!(output, "```md").unwrap();

    for dbuser in usersresults {
        let userid = dbuser.parse_id();
        let mut username = None;

        let mut user_errors = String::new();

        match msg.guild_id.unwrap().member(&ctx, userid) {
            Ok(member) => {
                username.get_or_insert(member.user.read().tag());
                for class in dbuser.parse_classes(db) {
                    let role = class.parse_role();

                    if !member.roles.iter().any(|r| r == &role) {
                        let role_name = role
                            .to_role_cached(&ctx)
                            .map(|r| r.name)
                            .unwrap_or(role.to_string());

                        if role_name == class.name {
                            writeln!(
                                user_errors,
                                "User is missing the role for the class `{}`",
                                role_name
                            )
                            .unwrap();
                        } else {
                            writeln!(
                                user_errors,
                                "User is missing the role `{}` for the class `{}`",
                                role_name, class.name
                            )
                            .unwrap();
                        }
                    }
                }
            }
            Err(serenity::Error::Http(e)) => match *e {
                serenity::http::HttpError::UnsuccessfulRequest(_) => {
                    writeln!(user_errors, "User does not exist or is not in this server").unwrap()
                }
                _ => writeln!(user_errors, "{}", e.description().to_owned()).unwrap(),
            },
            Err(e) => writeln!(user_errors, "{}", e.description().to_owned()).unwrap(),
        }

        if user_errors.len() > 0 {
            writeln!(
                output,
                "# {}\n{}",
                username.unwrap_or(userid.to_string()),
                user_errors
            )
            .unwrap();
        }
    }

    writeln!(output, "```").unwrap();

    writeln!(output, "> **Classes Database Errors**").unwrap();
    writeln!(output, "```md").unwrap();

    for class in classesresults.iter() {
        let mut class_errors = String::new();

        match class.parse_role().to_role_cached(&ctx) {
            Some(_) => {},
            None => writeln!(class_errors, "Role {} does not exist", class.role).unwrap()
        }

        if class_errors.len() > 0 {
            writeln!(
                output,
                "# {}\n{}",
                class.name,
                class_errors
            )
            .unwrap();
        }
    }

    writeln!(output, "```").unwrap();

    writeln!(output, "> **Orphaned Users**").unwrap();
    writeln!(output, "```md").unwrap();

    // for member in msg
    //     .guild_id
    //     .unwrap()
    //     .members_iter(&ctx)
    //     .filter_map(|x| x.ok())
    //     .filter(|x| x.roles.iter().any(|role| role == &config.roles.tester))
    // {
    //     writeln!(output, "# {}", member.display_name()).unwrap();
    //     for role in member.roles {
    //         if classesresults.iter().any(|y| y.parse_role() == role) {
    //             writeln!(output, "{}", role).unwrap();
    //         }
    //     }
    // }
    writeln!(output, "// TODO: FIXME:").unwrap();

    writeln!(output, "```").unwrap();

    let chunks = output.as_bytes().chunks(2000);

    for chunk in chunks {
        msg.channel_id.say(&ctx, String::from_utf8_lossy(chunk))?;
    }

    Ok(())
}
