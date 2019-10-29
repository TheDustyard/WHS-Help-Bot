use crate::bot_data::{BotConfig, BotLogger, DatabaseConnection};
use crate::commands::checks::ADMIN_CHECK;
use crate::config::StaticConfiguration;
use crate::db::Database;
use crate::util::SearchHighlighter;
use serenity::{
    framework::standard::{
        macros::{command, group},
        ArgError, Args, CommandResult,
    },
    model::{channel::Message, id::RoleId},
    prelude::*,
    utils::Colour,
};
use std::convert::TryInto;

group!({
    name: "Classes",
    options: {
        description: "Class management commands",
        prefixes: ["classes", "c", "cl"],
        // default_command: list
    },
    commands: [list/*, add, remove, edit, mine, join, leave */],
});

#[command]
#[description = "List the classes that are avaliable"]
#[usage = "<filter>"]
#[example = "History"]
#[min_args(0)]
#[max_args(1)]
fn list(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    let db: &Database = &data.get::<DatabaseConnection>().unwrap().lock().unwrap();

    let filter = args.remains();

    let all_count: usize = db.classes_count()?.try_into().unwrap();

    let classes = match &filter {
        Some(filter) => db.search_classes(filter),
        None => db.get_all_classes(),
    };

    let highlight = match &filter {
        Some(f) => Some(SearchHighlighter::new(f)),
        None => None,
    };

    match classes {
        Ok(classes) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    if all_count == classes.len() {
                        e.title(format!("Displaying {} classes", classes.len()));
                    } else {
                        e.title(format!(
                            "Displaying {} of {} classes",
                            classes.len(),
                            all_count
                        ));
                    }

                    let classmap = Database::map_classes_by_group(&classes);

                    for (group, classes) in classmap {
                        e.field(
                            group
                                .map(|x| {
                                    format!(
                                        "{} {} {}",
                                        if let Some(h) = &highlight {
                                            h.highlight(&x.name)
                                        } else {
                                            x.name
                                        },
                                        x.channel_group,
                                        x.vc
                                    )
                                })
                                .unwrap_or("No Group".to_owned()),
                            classes
                                .into_iter()
                                .map(|class| {
                                    format!(
                                        "**{}**\nRole: {}\nChannel: {}\nId: `{}`",
                                        if let Some(h) = &highlight {
                                            h.highlight(&class.name)
                                        } else {
                                            class.name.clone()
                                        },
                                        class.role,
                                        class.channel,
                                        class.id
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join("\n"),
                            true,
                        );
                    }

                    if classes.is_empty() {
                        e.color(Colour::DARK_GOLD);
                    } else {
                        e.color(Colour::DARK_GREEN);
                    }

                    if let Some(filter) = &filter {
                        e.footer(|footer| footer.text(format!("Filter: `{}`", filter)));
                    }

                    e
                });

                m
            })?;
        }
        Err(err) => {
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Error Loading Classes");
                    e.color(Colour::DARK_RED);
                    e.description(format!("```{:#?}```", err));

                    e
                });
                m
            })?;
        }
    };

    Ok(())
}
