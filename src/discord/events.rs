use crate::bot_data::{BotConfig, BotLogger};
use log::info;
use serenity::{
    model::{guild::Action, prelude::*},
    prelude::*,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

const IS_READY: AtomicBool = AtomicBool::new(false);

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data_about_bot: Ready) {
        if IS_READY.load(Ordering::Relaxed) {
            return;
        }
        IS_READY.store(true, Ordering::Relaxed);

        #[cfg(debug_assertions)]
        ctx.set_presence(
            Some(Activity::playing("with zach's emotions")),
            OnlineStatus::DoNotDisturb,
        );

        #[cfg(not(debug_assertions))]
        ctx.set_presence(Some(Activity::listening("!help")), OnlineStatus::Online);

        let mode = if cfg!(debug_assertions) {
            "development"
        } else {
            "release"
        };

        info!(
            "{} ready! Status has been to reflect the bot running in {} mode",
            data_about_bot.user.tag(),
            mode
        );

        #[cfg(not(debug_assertions))]
        {
            let data = ctx.data.read();

            let logger = data.get::<BotLogger>().unwrap();
            let config = data.get::<BotConfig>().unwrap();

            let _ = logger.info(
                &ctx,
                format!("Successfully started"),
                format!(
                    "The bot is running in {} mode. For information on using the bot please visit {}.",
                    mode, config.bot.website
                ),
            );
        }
    }

    // TODO: ADD MORE FIX
    fn guild_role_create(&self, ctx: Context, guild_id: GuildId, new: Role) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        // let auditlog = guild_id
        //     .audit_logs(&ctx, Some(31), None, None, None)
        //     .unwrap();

        // for (id, entry) in auditlog.entries {
        //     // if let Action::Role(role) = entry.action {}
        //     info!("{} {} {:?} {}", id, entry.target_id, entry.action, new.id);
        // }

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A new role {} was created.\nIf this is meant to be a class, please do not add such roles manually, use the `!c import` command to add this role to the classes databse and please use the `!c create` command from now on to create classes.\n*For more information on using the bot please visit {}*", new, config.bot.website),
        );
    }
    fn guild_role_delete(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        removed_role_id: RoleId,
        _removed_role_data_if_available: Option<Role>,
    ) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A role, {}, was deleted.\nIf this is registered as a class, please do not delete such roles manually. Use the `!c delete` command from now on to delete classes.\n*For more information on using the bot please visit {}*", removed_role_id, config.bot.website),
        );
    }
    fn guild_role_update(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        _old_data_if_available: Option<Role>,
        new: Role,
    ) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A role, {}, was updated.\nIf this is registered as a class, please do not update such roles manually. Use the `!c edit` command from now on to edit classes.\n*For more information on using the bot please visit {}*", new, config.bot.website),
        );
    }
    fn channel_create(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A channel, {}, was created.\nIf this should be tied to a class, please do not delete such roles manually. Use the `!c create` command from now on to create classes and their channels.\n*For more information on using the bot please visit {}*", channel.read(), config.bot.website),
        );
    }
    fn channel_delete(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A channel, {}, was deleted.\nIf this was registered with a class, please do not update such channels manually. Use the `!c delete` command from now on to delete classes.\n*For more information on using the bot please visit {}*", channel.read(), config.bot.website),
        );
    }
    fn channel_update(&self, ctx: Context, _old: Option<Channel>, new: Channel) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A channel, {}, was updated.\nIf this is registered with a class, please do not update such channels manually. Use the `!c edit` command from now on to edit classes and their respective channels.\n*For more information on using the bot please visit {}*", new, config.bot.website),
        );
    }
    fn category_create(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A channel category, {}, was created.\nIf this should be registered with a group, please do not create such categories manually. Use the `!g create` command from now on to create groups.\n*For more information on using the bot please visit {}*", category.read().id, config.bot.website),
        );
    }
    fn category_delete(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();
        let config = data.get::<BotConfig>().unwrap();

        let _ = logger.error(
            &ctx,
            "PARANOIA",
            format!("**WATCH OUT!** A channel category, {}, was deleted.\nIf this is registered as a group, please do not delete such categories manually. Use the `!g delete` or `!g edit` commands from now on to delete or rename groups.\n*For more information on using the bot please visit {}*", category.read().id, config.bot.website),
        );
    }
}
