use crate::bot_data::SqliteDatabaseConnection;
use crate::db::models::DatabaseClass;
use crate::db::schema::classes as database_classes;
use diesel::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::*,
};

#[command]
#[description = "Join a class."]
#[usage = "<id or name>"]
#[example = "609773945796821022"]
#[num_args(1)]
#[only_in(guilds)]
fn join(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();

    let db: &SqliteConnection = &data
        .get::<SqliteDatabaseConnection>()
        .unwrap()
        .lock()
        .unwrap();

    let class_id = args.single::<RoleId>();

    match class_id {
        Ok(class_id) => {
            match database_classes::table
                .filter(database_classes::id.eq(class_id.to_string()))
                .load::<DatabaseClass>(db)
            {
                Ok(_) => match msg.member(&ctx.cache).unwrap().add_role(&ctx, class_id) {
                    Ok(_) => msg.channel_id.say(
                        &ctx,
                        format!("Successfully joined class `{}`", class_id.to_string()),
                    )?,
                    Err(e) => msg.channel_id.say(
                        &ctx,
                        format!(
                            "Failed to join class `{}`\n```{:?}```",
                            class_id.to_string(),
                            e
                        ),
                    )?,
                },
                Err(_) => msg.channel_id.say(
                    &ctx,
                    format!("Could not find class with id `{}`", class_id.to_string()),
                )?,
            };
        }
        Err(_) => {
            args.rewind();
            let class_name = args.single_quoted::<String>()?;

            match database_classes::table
                .load::<DatabaseClass>(db)?
                .into_iter()
                .filter(|class| {
                    class
                        .parse_role_id()
                        .to_role_cached(&ctx)
                        .map(|r| r.name.to_lowercase() == class_name.to_lowercase())
                        .unwrap_or(false)
                })
                .next()
            {
                Some(class) => match msg
                    .member(&ctx.cache)
                    .unwrap()
                    .add_role(&ctx, class.parse_role_id())
                {
                    Ok(_) => msg
                        .channel_id
                        .say(&ctx, format!("Successfully joined class `{}`", class_name))?,
                    Err(e) => msg.channel_id.say(
                        &ctx,
                        format!("Failed to join class `{}`\n```{:?}```", class_name, e),
                    )?,
                },
                None => msg.channel_id.say(
                    &ctx,
                    format!("Could not find class with name `{}`", class_name),
                )?,
            };
        }
    }

    Ok(())
}
