use crate::bot_data::SqliteDatabaseConnection;
use crate::db::models::{DatabaseClass, DatabaseUser};
use diesel::prelude::*;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::model::misc::UserIdParseError;
use serenity::prelude::*;
use std::error::Error;
use textwrap::{fill, indent};

#[command]
#[description = "Check for errors and problems with the user listing."]
#[usage = "[filter]"]
#[only_in(guilds)]
fn errors(ctx: &mut Context, msg: &Message) -> CommandResult {
    use crate::db::schema::{classes, users};

    let data = ctx.data.read();

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

    msg.channel_id.say(
        &ctx,
        format!(
            "> **Database Errors**\n```md\n{}```\n> **Orphaned Users**\n```md\n{:?}```",
            usersresults
                .iter()
                .map(|dbuser: &DatabaseUser| -> (_, Option<String>) {
                    match dbuser.get_id() {
                        Ok(id) => match id.to_user(&ctx) {
                            Ok(user) => {
                                let errors: Vec<_> = dbuser
                                    .get_classes(db)
                                    .iter()
                                    .map(|class| match class {
                                        Ok(class) => match class.get_id() {
                                            Ok(_) => None,
                                            Err(_) => Some(format!(
                                                "Class {} has a malformed ID",
                                                class.id
                                            )),
                                        },
                                        Err(e) => Some(format!("Could not find class: {:?}", e)),
                                    })
                                    .filter_map(|x| x)
                                    .collect();

                                if errors.len() > 0 {
                                    return (
                                        dbuser,
                                        Some(
                                            errors
                                                .iter()
                                                .map(|error| indent(&fill(error, 40), "    "))
                                                .collect::<Vec<_>>()
                                                .join("\n"),
                                        ),
                                    );
                                }

                                (dbuser, None)
                            }
                            Err(serenity::Error::Http(e)) => match *e {
                                serenity::http::HttpError::UnsuccessfulRequest(_) => (
                                    dbuser,
                                    Some(
                                        "    User does not exist or is not in this server"
                                            .to_owned(),
                                    ),
                                ),
                                _ => (dbuser, Some(e.description().to_owned())),
                            },
                            Err(e) => (dbuser, Some(e.description().to_owned())),
                        },
                        Err(UserIdParseError::InvalidFormat) => {
                            (dbuser, Some("    Unble to parse UserID".to_owned()))
                        }
                    }
                })
                .filter(|x| x.1.is_some())
                .map(|x| format!("# {}\n{}", x.0.name, x.1.unwrap()))
                .collect::<Vec<_>>()
                .join("\n"),
            msg.guild_id
                .unwrap()
                .members_iter(&ctx)
                .filter_map(|x| x.ok())
                .filter(
                    |member| member.roles.iter().any(|x| classesresults.iter().any(|y| y
                        .get_role()
                        .unwrap()
                        .0
                        == x.0))
                )
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    )?;

    Ok(())
}
