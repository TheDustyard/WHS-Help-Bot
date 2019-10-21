use log::debug;
use serenity::{
    model::{
        gateway::{Activity, Ready},
        user::OnlineStatus,
    },
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        #[cfg(debug_assertions)]
        ctx.set_presence(
            Some(Activity::playing("with zach's emotions")),
            OnlineStatus::DoNotDisturb,
        );

        #[cfg(not(debug_assertions))]
        ctx.set_presence(Some(Activity::listening("!help")), OnlineStatus::Online);

        debug!("Bot ready! Status has been set");
    }
}
