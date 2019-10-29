use crate::db::Database;
use crate::model::Class;
use crate::util::SearchHighlighter;
use serenity::{builder::CreateEmbed, utils::Colour};

pub fn create_embed_for_classes<'a>(
    embed: &'a mut CreateEmbed,
    all_count: usize,
    classes: &[Class],
    filter: Option<&str>,
) -> &'a mut CreateEmbed {
    let highlight = match &filter {
        Some(f) => Some(SearchHighlighter::new(f)),
        None => None,
    };
    if all_count == classes.len() {
        embed.title(format!("Displaying {} classes", classes.len()));
    } else {
        embed.title(format!(
            "Displaying {} of {} classes",
            classes.len(),
            all_count
        ));
    }

    let classmap = Database::map_classes_by_group(&classes);

    for (group, classes) in classmap {
        embed.field(
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

    embed.color(if classes.is_empty() {
        Colour::DARK_GOLD
    } else {
        Colour::DARK_GREEN
    });

    if let Some(filter) = &filter {
        embed.footer(|footer| footer.text(format!("Filter: `{}`", filter)));
    }

    embed
}
