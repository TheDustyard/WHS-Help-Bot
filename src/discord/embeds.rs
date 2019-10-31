use crate::db::Database;
use crate::model::{Class, Group};
use serenity::{builder::CreateEmbed, model::misc::Mentionable, utils::Colour};

pub fn create_embed_for_classes<'a>(
    embed: &'a mut CreateEmbed,
    all_count: usize,
    classes: &[Class],
    filter: Option<&str>,
    detailed: bool,
) -> &'a mut CreateEmbed {
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
                .map(|x| format!("{}", x.name))
                .unwrap_or("No Group".to_owned()),
            classes
                .into_iter()
                .map(|class| {
                    if detailed {
                        format!(
                            "**{}**
                        Role:
                        > Mention: {}
                        > Id: `{}`
                        Channel:
                        > Mention: {}
                        > Id: `{}`
                        Id: `{}`",
                            class.name.clone(),
                            class.role.mention(),
                            class.role,
                            class.channel.mention(),
                            class.channel,
                            class.id
                        )
                    } else {
                        format!(
                            "**{}**
                        Role: {}
                        Channel: {}
                        Id: `{}`",
                            class.name.clone(),
                            class.role.mention(),
                            class.channel.mention(),
                            class.id
                        )
                    }
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

pub fn create_embed_for_groups<'a>(
    embed: &'a mut CreateEmbed,
    all_count: usize,
    groups: &[Group],
    filter: Option<&str>,
    detailed: bool,
) -> &'a mut CreateEmbed {
    if all_count == groups.len() {
        embed.title(format!("Displaying {} groups", groups.len()));
    } else {
        embed.title(format!(
            "Displaying {} of {} groups",
            groups.len(),
            all_count
        ));
    };

    for group in groups {
        embed.field(
            &group.name,
            if detailed {
                format!(
                    "Channel Group:
                    > Mention - {}
                    > Id - `{}`
                    Voice Chat:
                    > Mention - {}
                    > Id - `{}`
                    Id: `{}`",
                    group.channel_group.mention(),
                    group.channel_group,
                    group.vc.mention(),
                    group.vc,
                    group.id
                )
            } else {
                format!(
                    "Channel Group: {}
                    Voice Chat: {}
                    Id: `{}`",
                    group.channel_group.mention(),
                    group.vc.mention(),
                    group.id
                )
            },
            true,
        );
    }

    embed.color(if groups.is_empty() {
        Colour::DARK_GOLD
    } else {
        Colour::DARK_GREEN
    });

    if let Some(filter) = &filter {
        embed.footer(|footer| footer.text(format!("Filter: `{}`", filter)));
    }

    embed
}
