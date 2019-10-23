use crate::bot_data::BotLogger;
use log::info;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data_about_bot: Ready) {
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

        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();

        let _ = logger.info(
            &ctx,
            format!("Successfully started"),
            format!("The bot is running in {} mode. For information on using the bot please visit URLGOESHERE.", mode),
        );
    }

    fn guild_role_create(&self, ctx: Context, _guild_id: GuildId, new: Role) {
        let data = ctx.data.read();

        let logger = data.get::<BotLogger>().unwrap();

        let _ = logger.warn(
            &ctx,
            format!("PARANOIA"),
            format!("WATCH OUT! A new role {} was created. If this is meant to be a class, please do not add such roles manually, use the `import` command to add this role to the classes databse and please use the `create` command from now on to create classes. For more information on using the bot please visit URLGOESGERE", new),
        );
    }
    fn guild_role_delete(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _removed_role_id: RoleId,
        _removed_role_data_if_available: Option<Role>,
    ) {
    }
    fn guild_role_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _old_data_if_available: Option<Role>,
        _new: Role,
    ) {
    }
    fn channel_create(&self, _ctx: Context, _channel: Arc<RwLock<GuildChannel>>) {}
    fn category_create(&self, _ctx: Context, _category: Arc<RwLock<ChannelCategory>>) {}
    fn category_delete(&self, _ctx: Context, _category: Arc<RwLock<ChannelCategory>>) {}
    fn channel_delete(&self, _ctx: Context, _channel: Arc<RwLock<GuildChannel>>) {}
    fn channel_update(&self, _ctx: Context, _old: Option<Channel>, _new: Channel) {}
}
